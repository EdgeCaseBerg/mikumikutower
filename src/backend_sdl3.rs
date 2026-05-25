use crate::Rect;
use crate::asset_loader::AssetLoader;
use crate::audio::Audio;
use crate::backend::*;
use crate::constants::*;
use crate::constants::{MusicId, SfxId};
use crate::game::Game;
use crate::game::GameContext;
use crate::game_options::GameOptions;
use crate::renderer::{Color, RenderCommand, Renderer};

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::rc::Rc;
use std::time::Duration;
use std::time::Instant;

use sdl3::AudioSubsystem;
use sdl3::EventPump;
use sdl3::Sdl;
use sdl3::VideoSubsystem;
use sdl3::audio::AudioFormat;
use sdl3::audio::AudioSpec;
use sdl3::audio::AudioSpecWAV;
use sdl3::audio::AudioStreamOwner;
use sdl3::event::{Event, WindowEvent};
use sdl3::filesystem::get_current_directory;
use sdl3::filesystem::{GlobFlags, glob_directory};
use sdl3::image::LoadTexture;
use sdl3::keyboard::Keycode;
use sdl3::mouse::MouseButton;
use sdl3::render::Texture;
use sdl3::render::TextureCreator;
use sdl3::render::WindowCanvas;
use sdl3::video::WindowContext;

pub struct BackendSDL3 {
    sdl: Sdl,
}

type SDL3Texture = Texture<'static>;

pub struct SDL3Context {
    // Note: textures MUST be declared ABOVE window_canvas because
    // drop order is top to bottom and all textures need to be dropped
    // BEFORE the canvas is dropped
    textures: SDL3Textures,
    window_canvas: WindowCanvas,
    video: VideoSubsystem,
    audio: AudioSubsystem,
}

struct SoundData {
    spec: AudioSpecWAV,
    duration: Duration,
}

struct Bucket {
    spec: Spec,
    streams: Vec<SfxStream>,
}

struct SfxStream {
    stream: AudioStreamOwner,
    free_at: Option<Instant>,
}

// This hack brought to you by AudioSpec not implementing Hash.
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Spec {
    freq: i32,
    channels: i32,
    format: sdl3::audio::AudioFormat,
}

enum MusicTrack {
    A,
    B,
}

struct SDL3Sounds {
    sound_by_id: HashMap<SfxId, SoundData>,
    music_by_id: HashMap<MusicId, SoundData>,
    buckets: Vec<Bucket>,
    poolsize: usize,
    base_path: PathBuf,
    current_track: MusicTrack,
    music_streams: [Option<SfxStream>; 2],
    context: Rc<RefCell<SDL3Context>>,
}

fn to_hashable_spec(a: &AudioSpecWAV) -> Spec {
    Spec {
        freq: a.freq,
        channels: a.channels.into(),
        format: a.format,
    }
}

fn spec_duration(spec: &AudioSpecWAV) -> Duration {
    let bytes_per_sample = match spec.format {
        AudioFormat::U8 | AudioFormat::S8 => 1,
        AudioFormat::S16LE | AudioFormat::S16BE => 2,
        AudioFormat::S32LE | AudioFormat::S32BE | AudioFormat::F32LE | AudioFormat::F32BE => 4,
        _ => 2,
    };
    let total_samples = spec.buffer().len() / (bytes_per_sample * spec.channels as usize);
    let seconds = (total_samples as f64) / spec.freq as f64;
    Duration::from_secs_f64(seconds)
}

impl SDL3Sounds {
    fn new(context: Rc<RefCell<SDL3Context>>, game_options: &GameOptions) -> Self {
        let buckets = vec![];
        let base_path = get_current_directory()
            .expect("cant get base path for audio player")
            .join(game_options.assets_path.clone());

        Self {
            buckets,
            sound_by_id: HashMap::new(),
            music_by_id: HashMap::new(),
            poolsize: game_options.audio_pool_size,
            base_path,
            current_track: MusicTrack::A,
            music_streams: [None, None],
            context,
        }
    }
    // TODO: explain in blog about how annoying it is that I cant reuse it
    fn get_sound_buffer(&self, sound_id: SfxId) -> Option<&SoundData> {
        self.sound_by_id.get(&sound_id)
    }
}

impl SfxStream {
    fn is_free(&self, now: Instant) -> bool {
        self.free_at.map_or(true, |t| now >= t)
    }

    fn claim(&mut self, entry: &SoundData, now: Instant) {
        let _ = self.stream.clear();
        let _ = self.stream.put_data(entry.spec.buffer());
        let _ = self.stream.resume();
        self.free_at = Some(now + entry.duration);
    }
}

impl Audio for SDL3Sounds {
    fn play_sfx(&mut self, id: SfxId) {
        // TODO should we take this instant in from above?
        let now = Instant::now();
        let Some(sound_data) = self.sound_by_id.get(&id) else {
            return;
        };
        // find the bucket
        let bucket_key = to_hashable_spec(&sound_data.spec);
        let Some(bucket) = self.buckets.iter_mut().find(|b| b.spec == bucket_key) else {
            eprintln!("No bucket found for spec {:?}", bucket_key);
            return;
        };
        let stream = if let Some(stream) = bucket.streams.iter_mut().find(|s| s.is_free(now)) {
            stream
        } else {
            // All busy — steal the one that will free soonest
            bucket.streams.iter_mut().min_by_key(|s| s.free_at).unwrap()
        };
        stream.claim(&sound_data, now);
    }
    fn load_sfx(&mut self, sound_id: SfxId) {
        if !self.sound_by_id.get(&sound_id).is_none() {
            return;
        }

        // TODO I suppose make things turn results and ? it all.
        let path = self.base_path.join(sfx_id_to_relative_path(sound_id));
        let spec = AudioSpecWAV::load_wav(path).expect("could not load spec from path");
        let data = SoundData {
            duration: spec_duration(&spec),
            spec,
        };
        self.sound_by_id.insert(sound_id, data);
    }
    fn play_music(&mut self, id: MusicId) {
        let Some(sound_data) = self.music_by_id.get(&id) else {
            return;
        };
        let (play_index, pause_index) = match self.current_track {
            MusicTrack::A => (0, 1),
            MusicTrack::B => (1, 0),
        };
        let now = Instant::now();
        self.music_streams[play_index] = Some({
            let ctx = &mut *self.context.borrow_mut();
            let device = ctx.audio.default_playback_device();
            let mut stream = SfxStream {
                stream: device
                    .open_device_stream(
                        Some(AudioSpec {
                            freq: Some(sound_data.spec.freq),
                            channels: Some(sound_data.spec.channels.into()),
                            format: Some(sound_data.spec.format),
                        })
                        .as_ref(),
                    )
                    .expect("could not open logical device for spec"),
                free_at: Some(now),
            };
            stream.claim(&sound_data, now);
            stream
        });

        match &mut self.music_streams[pause_index] {
            None => {}
            Some(SfxStream { stream, .. }) => {
                let _ = stream.pause();
            }
        }
        self.current_track = match self.current_track {
            MusicTrack::A => MusicTrack::B,
            MusicTrack::B => MusicTrack::A,
        };
    }
    fn load_music(&mut self, id: MusicId) {
        if !self.music_by_id.get(&id).is_none() {
            return;
        }
        // TODO I suppose make things turn results and ? it all.
        let path = self.base_path.join(music_id_to_relative_path(id));
        let spec = AudioSpecWAV::load_wav(path).expect("could not load spec from path");
        let data = SoundData {
            duration: spec_duration(&spec),
            spec,
        };
        self.music_by_id.insert(id, data);
    }
    fn load_bg_music(&mut self) -> Vec<MusicId> {
        let user_wav_folder = self.base_path.join("audio").join("cc-vocaloid");
        let mut ids = Vec::new();
        if let Ok(results) =
            glob_directory(user_wav_folder, Some("*.wav"), GlobFlags::CASEINSENSITIVE)
        {
            for path in &results {
                let filename = path.file_name();
                if filename.is_none() {
                    continue;
                }
                let filename = filename.unwrap().to_str().unwrap();
                let desired_id = filename[0..filename.len() - 4].parse::<usize>();
                if desired_id.is_ok() {
                    let music_id = MusicId(desired_id.unwrap());
                    ids.push(music_id);
                    self.load_music(music_id);
                }
            }
        }
        ids
    }
    fn music_duration_seconds(&self, id: MusicId) -> Duration {
        let Some(sound_data) = self.music_by_id.get(&id) else {
            return Duration::from_secs_f64(0.0);
        };
        spec_duration(&sound_data.spec)
    }

    fn prepare(&mut self) {
        let mut specs_to_prepare: HashSet<Spec> = self
            .sound_by_id
            .values()
            .map(|v| to_hashable_spec(&v.spec))
            .collect();
        for spec in self.music_by_id.values().map(|v| to_hashable_spec(&v.spec)) {
            specs_to_prepare.insert(spec);
        }

        // Clean out anything we DONT need anymore:
        let mut already_exist = HashSet::new();
        self.buckets.retain_mut(|bucket| {
            let exists = specs_to_prepare.contains(&bucket.spec);
            if exists {
                already_exist.insert(bucket.spec.clone());
            }
            for SfxStream { stream, .. } in &bucket.streams {
                let _ = stream.pause();
            }
            exists
        });

        let ctx = &mut *self.context.borrow_mut();
        for spec_needs_bucket in specs_to_prepare.difference(&already_exist) {
            let mut streams = Vec::with_capacity(self.poolsize);
            for _ in 0..self.poolsize {
                let device = ctx.audio.default_playback_device();
                let stream = SfxStream {
                    stream: device
                        .open_device_stream(
                            Some(AudioSpec {
                                freq: Some(spec_needs_bucket.freq),
                                channels: Some(spec_needs_bucket.channels),
                                format: Some(spec_needs_bucket.format),
                            })
                            .as_ref(),
                        )
                        .expect("could not open logical device for spec"),
                    free_at: None,
                };
                streams.push(stream);
            }
            self.buckets.push(Bucket {
                spec: *spec_needs_bucket,
                streams,
            })
        }
    }
}

pub struct SDL3Textures {
    texture_by_id: HashMap<TextureId, SDL3Texture>,
    // ORDER OF STRUCT IS IMPORTANT BECAUSE OF DROP ORDER
    // WE DROP THE TEXTURES PRIOR TO THE CREATOR GOING AWAY
    texture_creator: TextureCreator<WindowContext>,
}

impl SDL3Textures {
    fn from(texture_creator: TextureCreator<WindowContext>) -> Self {
        SDL3Textures {
            texture_creator,
            texture_by_id: HashMap::new(),
        }
    }

    fn get_texture(&self, texture_id: TextureId) -> Option<&SDL3Texture> {
        self.texture_by_id.get(&texture_id)
    }

    fn load(&mut self, id: TextureId, path: PathBuf) {
        let tex = self.texture_creator.load_texture(path).unwrap();
        let tex = make_static(tex);
        self.texture_by_id.insert(id, tex);
    }
}

struct AssetLoaderSDL3 {
    context: Rc<RefCell<SDL3Context>>,
    base_path: PathBuf,
}

impl AssetLoaderSDL3 {
    fn new(context: Rc<RefCell<SDL3Context>>, game_options: &GameOptions) -> Self {
        let base = get_current_directory().expect("cant get base path");
        let base = base.join(game_options.assets_path.clone());
        Self {
            context,
            base_path: base,
        }
    }
}

impl AssetLoader for AssetLoaderSDL3 {
    fn ensure_texture_spritesheet_loaded(&mut self, id: TextureId) {
        let ctx = &mut *self.context.borrow_mut();
        if !ctx.textures.get_texture(id).is_none() {
            return;
        }
        let asset_path = id_to_relative_path(id);
        let asset_path = self.base_path.join(asset_path);
        ctx.textures.load(id, asset_path);
    }
}

// Alchemy! we do this to shunt off the lifetime the sdl3 lib sets on the textures.
// both it and we know that its lifetime is tied to the texture_creator, but they
// didn't represent this by defining a lifetime on the creator, and we don't need to
// care or worry about this because the texture_creator is owned by SDL3Textures and
// so when it goes out of scope it can drop everything. I imagine I might need to
// implement a Drop for SDL3Textures to make sure that happens, but then again, its
// dropping point is _probably_ going to be the end of the program so... eh.
// The SDL3 docs says we should destroy it when we're done https://wiki.libsdl.org/SDL3_image/IMG_LoadTexture
//
fn make_static(tex: Texture) -> Texture<'static> {
    unsafe { std::mem::transmute(tex) }
}

impl BackendSDL3 {
    pub fn new(_game_options: &GameOptions) -> Self {
        let sdl_handle = sdl3::init().expect("failed to init SDL");
        BackendSDL3 { sdl: sdl_handle }
    }
}

impl Backend for BackendSDL3 {
    fn create_event_loop(&self, game_options: &GameOptions) -> Box<dyn BackendEventLoop> {
        let event_pump = self.sdl.event_pump().unwrap();

        let video_subsystem = self.sdl.video().expect("failed to get video context");
        let audio_subsystem = self.sdl.audio().expect("failed to get audio context");

        // Side note, window to borderless and all that would need to re-create window and derived canvases
        let window = video_subsystem
            .window(
                &game_options.name,
                game_options.window_width,
                game_options.window_height,
            )
            .position_centered()
            .resizable()
            .build()
            .expect("failed to build window");

        let canvas = window.into_canvas();
        let textures = SDL3Textures::from(canvas.texture_creator());

        // If we end up having some custom form of cursor for each scene then we can do this
        // self.sdl.mouse().show_cursor(false);

        let e = EventLoopSDL3 {
            event_pump,
            context: Rc::new(RefCell::new(SDL3Context {
                video: video_subsystem,
                window_canvas: canvas,
                textures,
                audio: audio_subsystem,
            })),
        };
        Box::new(e)
    }
}

pub struct EventLoopSDL3 {
    event_pump: EventPump,
    context: Rc<RefCell<SDL3Context>>, // in a rc + refcell because we need to be able to pass around &mut for shared stuff.
}

impl BackendEventLoop for EventLoopSDL3 {
    fn run(&mut self, game: &mut Game, game_context: &mut GameContext) {
        let scene = game.scene.as_mut();
        if let Some(scene) = scene {
            scene.init(game_context);
        }

        // initialize the audio pool if the scene has queued things up
        let audio = game_context.audio.as_mut();
        if let Some(audio) = audio {
            audio.prepare();
        }

        'running: loop {
            // TODO: merge events into state tracking system that doesn't exist yet
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::MouseMotion {
                        mousestate, x, y, ..
                    } => {
                        game_context.mouse_context.update(
                            mousestate.left(),
                            mousestate.right(),
                            Some((x, y)),
                        );
                    }
                    Event::MouseButtonDown {
                        mouse_btn, x, y, ..
                    } => {
                        game_context.mouse_context.update(
                            mouse_btn == MouseButton::Left,
                            mouse_btn == MouseButton::Right,
                            Some((x, y)),
                        );
                    }
                    Event::Window { win_event, .. } => match win_event {
                        WindowEvent::Resized(w, h) => {
                            game_context.screen_size = (w as u32, h as u32);
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
            game.update(game_context);
            if let Some(mut next_scene) = game_context.next_scene.take() {
                next_scene.init(game_context);
                game.scene = Some(next_scene);
                game.reset_for_next_scene();
                let audio = game_context.audio.as_mut();
                if let Some(audio) = audio {
                    audio.prepare();
                }
            }
            game.draw(game_context);
            if game_context.shutdown_flag {
                break;
            }
        }
    }

    fn new_renderer(&self, _game_options: &GameOptions) -> Box<dyn Renderer> {
        let r = RendererSDL3 {
            context: self.context.clone(),
            commands: Vec::with_capacity(32),
        };
        Box::new(r)
    }

    fn create_asset_loader(&self, game_options: &GameOptions) -> Box<dyn AssetLoader> {
        let a = AssetLoaderSDL3::new(self.context.clone(), game_options);
        Box::new(a)
    }

    fn create_audio(&self, game_options: &GameOptions) -> Box<dyn Audio> {
        let s = SDL3Sounds::new(self.context.clone(), game_options);
        Box::new(s)
    }
}

struct RendererSDL3 {
    context: Rc<RefCell<SDL3Context>>,
    commands: Vec<RenderCommand>,
}

impl RendererSDL3 {
    // Internally used before presenting. Drains all commands
    // in order to enque all the work to SDL3 that we want done
    // per frame.
    fn process_commands(&mut self) {
        for cmd in self.commands.drain(..) {
            match cmd {
                RenderCommand::DrawRect {
                    texture_id,
                    source,
                    destination,
                } => {
                    let ctx = &mut *self.context.borrow_mut();
                    if let Some(texture) = ctx.textures.get_texture(texture_id) {
                        let src: sdl3::rect::Rect = source.into();
                        let dst: sdl3::rect::Rect = destination.into();
                        ctx.window_canvas
                            .copy(texture, src, dst)
                            .unwrap_or_else(|_| {
                                let _ = &format!("failed to draw texture {}", texture_id.0);
                            });
                    }
                }
            }
        }
    }
}

impl Renderer for RendererSDL3 {
    fn name(&self) -> String {
        "SDL3 Renderer".to_string()
    }

    fn clear(&mut self, color: Color) {
        let mut ctx = self.context.borrow_mut();
        ctx.window_canvas.set_draw_color(color.to_sdl3());
        ctx.window_canvas.clear();
    }

    fn present(&mut self) {
        self.process_commands();
        let mut ctx = self.context.borrow_mut();
        ctx.window_canvas.present();
    }

    fn send_command(&mut self, cmd: RenderCommand) {
        self.commands.push(cmd);
    }
}

impl Color {
    pub fn to_sdl3(&self) -> sdl3::pixels::Color {
        let (r, g, b, a) = (*self).into();
        sdl3::pixels::Color::RGBA(r, g, b, a)
    }
}

impl From<Rect> for sdl3::rect::Rect {
    fn from(r: Rect) -> Self {
        sdl3::rect::Rect::new(r.x as i32, r.y as i32, r.width as u32, r.height as u32)
    }
}
