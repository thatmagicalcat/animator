use lerp::Lerp;
use sfml::system::*;

struct AnimateMove {
    starting_pos: Vector2f,
    ending_pos: Vector2f,

    steps: f32,
    step: f32,

    easing_fn: fn(f32) -> f32,
}

impl AnimateMove {
    fn new(
        starting_pos: impl Into<Vector2f>,
        ending_pos: impl Into<Vector2f>,
        time: Time,
        easing_fn: fn(f32) -> f32,
    ) -> Self {
        Self {
            starting_pos: starting_pos.into(),
            ending_pos: ending_pos.into(),
            step: 1.0 / time.as_seconds(),
            steps: 0.0,
            easing_fn,
        }
    }

    fn step(&mut self, dt: Time) {
        if self.steps > 1.0 {
            return;
        };

        self.steps += self.step * dt.as_seconds();
    }

    fn get_position(&self) -> Vector2f {
        let f = self.easing_fn;

        let x = self.starting_pos.x.lerp(self.ending_pos.x, f(self.steps));
        let y = self.starting_pos.y.lerp(self.ending_pos.y, f(self.steps));

        (x, y).into()
    }

    fn finished(&self) -> bool {
        self.steps > 1.0
    }
}

pub struct PathAnimation {
    starting_pos: Vector2f,
    ending_pos: Vector2f,

    animator: AnimateMove,
    stops: Vec<Vector2f>,
    current_stop_index: Option<usize>,
    distances: Vec<f32>,
    total_distance: f32,
    time: Time,

    easing_fn: fn(f32) -> f32,
}

impl PathAnimation {
    pub fn new(
        starting_pos: impl Into<Vector2f>,
        ending_pos: impl Into<Vector2f>,
        time: Time,
        stops: Vec<Vector2f>,
        easing_fn: fn(f32) -> f32,
    ) -> Self {
        let starting_pos = starting_pos.into();
        let ending_pos = ending_pos.into();

        let mut distances = Vec::with_capacity(1 + stops.len());
        let mut last_stop = &starting_pos;

        for current_stop in &stops {
            let distance = distance(&last_stop, current_stop);
            distances.push(distance);

            last_stop = current_stop;
        }

        distances.push(distance(&last_stop, &ending_pos));

        let total_distance: f32 = distances.iter().sum();

        Self {
            current_stop_index: if stops.is_empty() { None } else { Some(0) },
            animator: AnimateMove::new(
                starting_pos,
                if stops.is_empty() {
                    ending_pos
                } else {
                    stops[0]
                },
                // calculate how much time it will take to reach the first stop from starting point
                Time::seconds(distances[0] / total_distance * time.as_seconds()),
                easing_fn,
            ),

            stops,
            distances,
            starting_pos,
            ending_pos,
            time,
            total_distance,
            easing_fn,
        }
    }

    pub fn restart(&mut self) {
        self.current_stop_index = if self.stops.is_empty() { None } else { Some(0) };
        self.animator = AnimateMove::new(
            self.starting_pos,
            if self.stops.is_empty() {
                self.ending_pos
            } else {
                self.stops[0]
            },
            // calculate how much time it will take to reach the first stop from starting point
            Time::seconds(self.distances[0] / self.total_distance * self.time.as_seconds()),
            self.easing_fn,
        );
    }

    pub fn step(&mut self, dt: Time) {
        self.animator.step(dt);

        if self.animator.finished() && self.current_stop_index == None {
            // animation finished
            return;
        }

        // if the animation is finished
        // move towards the next stop
        if self.animator.finished() {
            self.current_stop_index.as_mut().map(|i| *i += 1);

            // if this was the last stop, move to the ending position
            if self.current_stop_index == Some(self.stops.len()) {
                self.current_stop_index = None;

                let current_stop = self.stops.last().unwrap();
                let time =
                    self.distances.last().unwrap() / self.total_distance * self.time.as_seconds();

                self.animator = AnimateMove::new(
                    *current_stop,
                    self.ending_pos,
                    Time::seconds(time),
                    self.easing_fn,
                );
            } else {
                let current_stop = self.stops[self.current_stop_index.unwrap() - 1];
                let next_stop = self.stops[self.current_stop_index.unwrap()];

                let time = self.distances[self.current_stop_index.unwrap()] / self.total_distance
                    * self.time.as_seconds();
                self.animator =
                    AnimateMove::new(current_stop, next_stop, Time::seconds(time), self.easing_fn);
            }
        }
    }

    pub fn finished(&self) -> bool {
        self.animator.finished() && self.current_stop_index == None
    }

    pub fn get_position(&self) -> Vector2f {
        if self.animator.finished() {
            return self.ending_pos;
        }

        self.animator.get_position()
    }
}

fn distance(a: &Vector2f, b: &Vector2f) -> f32 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;

    (dx * dx + dy * dy).sqrt()
}
