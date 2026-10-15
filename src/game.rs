use std::collections::VecDeque;
use std::time::Duration;

use rand::Rng;

pub type Pos = (u16, u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Playing,
    GameOver,
    Won,
}

pub struct Game {
    pub width: u16,
    pub height: u16,
    /// Head is at the front.
    pub snake: VecDeque<Pos>,
    pub food: Pos,
    pub score: u32,
    pub state: State,
    direction: Direction,
    /// Direction to apply on the next tick; prevents reversing via two quick key presses.
    next_direction: Direction,
}

impl Game {
    pub fn new(width: u16, height: u16) -> Self {
        let (cx, cy) = (width / 2, height / 2);
        let snake = VecDeque::from([(cx, cy), (cx - 1, cy), (cx - 2, cy)]);
        let mut game = Game {
            width,
            height,
            snake,
            food: (0, 0),
            score: 0,
            state: State::Playing,
            direction: Direction::Right,
            next_direction: Direction::Right,
        };
        game.spawn_food();
        game
    }

    pub fn turn(&mut self, dir: Direction) {
        if dir != self.direction.opposite() {
            self.next_direction = dir;
        }
    }

    /// Speeds up as the score grows, down to a floor of 60ms.
    pub fn tick_interval(&self) -> Duration {
        Duration::from_millis(150u64.saturating_sub(self.score as u64 * 3).max(60))
    }

    pub fn tick(&mut self) {
        if self.state != State::Playing {
            return;
        }
        self.direction = self.next_direction;

        let (x, y) = self.snake[0];
        let next = match self.direction {
            Direction::Up if y > 0 => (x, y - 1),
            Direction::Down if y + 1 < self.height => (x, y + 1),
            Direction::Left if x > 0 => (x - 1, y),
            Direction::Right if x + 1 < self.width => (x + 1, y),
            _ => {
                self.state = State::GameOver;
                return;
            }
        };

        let eating = next == self.food;
        if !eating {
            self.snake.pop_back();
        }
        // Checked after popping the tail, so moving into the cell the tail just left is allowed.
        if self.snake.contains(&next) {
            self.state = State::GameOver;
            return;
        }
        self.snake.push_front(next);

        if eating {
            self.score += 1;
            self.spawn_food();
        }
    }

    fn spawn_food(&mut self) {
        let total = self.width as usize * self.height as usize;
        if self.snake.len() >= total {
            self.state = State::Won;
            return;
        }
        let mut rng = rand::thread_rng();
        loop {
            let pos = (rng.gen_range(0..self.width), rng.gen_range(0..self.height));
            if !self.snake.contains(&pos) {
                self.food = pos;
                return;
            }
        }
    }
}
