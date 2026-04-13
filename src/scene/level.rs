use crate::Rect;

#[derive(Debug, Clone)]
enum TowerState {
    Ready,
    Cooldown { wait_for: u8, ticks_waited: u8 },
}

#[derive(Debug, Clone)]
struct Health {
    current: u8,
    max: u8,
}

impl Health {
    fn damage(&mut self, amount: u8) {
        self.current = self.current.saturating_sub(amount);
    }
}

#[derive(Debug)]
struct Base {
    position: Rect,
    health: Health,
}

#[derive(Debug, Clone)]
struct Tower {
    position: Rect,
    state: TowerState,
    range: u16, // 65535 should be enough
                // Add types later or some such thing.
}

impl Tower {
    fn basic(position: Rect) -> Self {
        Self {
            position,
            state: TowerState::Ready,
            range: 5, // TODO: revisit once we decide how big our gameboard is
        }
    }
}
