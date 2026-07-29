const MAX_ATTEMPS: u8 = 5;

#[derive(PartialEq, Debug)]
enum GuessResult {
    Correct,
    Partial,
    Wrong,
}

#[derive(PartialEq, Debug)]
enum GameState {
    Won,
    InProgress,
    Lost,
}

#[derive(PartialEq)]
struct Entry {
    artist: String,
    song: String,
}

struct Game {
    solution: Entry,
    state: GameState,
    max_attemps: u8,
    attempts: u8,
}

impl Game {
    pub fn new(solution: Entry) -> Self {
        Self {
            solution,
            max_attemps: MAX_ATTEMPS,
            attempts: 0,
            state: GameState::InProgress,
        }
    }
    pub fn check_guess(&mut self, guess: Entry) -> GuessResult {
        self.attempts = self.attempts + 1;
        if guess == self.solution {
            self.state = GameState::Won;
            return GuessResult::Correct;
        }

        if self.is_lost() {
            self.state = GameState::Lost
        }

        if self.same_artist(guess) {
            GuessResult::Partial
        } else {
            GuessResult::Wrong
        }
    }
    fn same_artist(&self, guess: Entry) -> bool {
        guess.artist == self.solution.artist && guess.song != self.solution.song
    }
    fn is_lost(&self) -> bool {
        self.attempts >= self.max_attemps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_guess() {
        let solution = Entry {
            artist: "Rick Astley".to_string(),
            song: "Never gonna give you up".to_string(),
        };
        let mut game = Game::new(solution);
        let guess = Entry {
            artist: "Rick Astley".to_string(),
            song: "Never gonna give you up".to_string(),
        };
        assert_eq!(GuessResult::Correct, game.check_guess(guess));
        assert_eq!(1, game.attempts);
        assert_eq!(GameState::Won, game.state);
    }

    #[test]
    fn partial_guess() {
        let solution = Entry {
            artist: "Rick Astley".to_string(),
            song: "Never gonna give you up".to_string(),
        };
        let mut game = Game::new(solution);
        let guess = Entry {
            artist: "Rick Astley".to_string(),
            song: "Together Forever".to_string(),
        };
        assert_eq!(GuessResult::Partial, game.check_guess(guess));
        assert_eq!(1, game.attempts);
        assert_eq!(GameState::InProgress, game.state)
    }

    #[test]
    fn wrong_guess() {
        let solution = Entry {
            artist: "Rick Astley".to_string(),
            song: "Never gonna give you up".to_string(),
        };
        let mut game = Game::new(solution);
        let guess = Entry {
            artist: "a-Ha".to_string(),
            song: "Take on Me".to_string(),
        };
        assert_eq!(GuessResult::Wrong, game.check_guess(guess));
        assert_eq!(1, game.attempts);
        assert_eq!(GameState::InProgress, game.state)
    }

    #[test]
    fn too_many_attemps() {
        let solution = Entry {
            artist: "Rick Astley".to_string(),
            song: "Never gonna give you up".to_string(),
        };
        let mut game = Game::new(solution);
        game.attempts = game.max_attemps;
        let guess = Entry {
            artist: "a-Ha".to_string(),
            song: "Take on Me".to_string(),
        };
        assert_eq!(GuessResult::Wrong, game.check_guess(guess));
        assert_eq!(game.max_attemps + 1, game.attempts);
        assert_eq!(GameState::Lost, game.state)
    }
}
