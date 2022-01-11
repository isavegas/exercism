using System;
using System.Collections.Generic;

public class HangmanState {
    public HangmanStatus Status { get; set; }
    public int RemainingGuesses { get; set; }
    public string MaskedWord { get; set; }
    public HashSet<char> Guesses { get; set; }
}

public enum HangmanStatus {
    Busy,
    Win,
    Lose
}

public class HangmanGame {
    public HangmanState State { get; set; }
    private string HiddenWord;

    public HangmanGame(string word) {
        HiddenWord = word;
        State = new HangmanState();
        State.Guesses = new HashSet<char>();
        State.RemainingGuesses = 9;
        State.MaskedWord = new String('_', HiddenWord.Length);
    }

    public void Start() {
        State.Status = HangmanStatus.Busy;
        StateChanged(this, State);
    }

    public void Guess(char c) {
        if (!State.Guesses.Contains(c)) {
            State.Guesses.Add(c);
            if (HiddenWord.Contains(c)) {
                int i = -1;
                char[] maskedWord = State.MaskedWord.ToCharArray();
                while (true) {
                    i = HiddenWord.IndexOf(c, i+1);
                    if (i > -1) {
                        maskedWord[i] = c;
                    } else {
                        break;
                    }
                }
                State.MaskedWord = new string(maskedWord);
                if (State.MaskedWord == HiddenWord) {
                    State.Status = HangmanStatus.Win;
                    StateChanged(this, State);
                }
                return;
            }
        }

        State.RemainingGuesses -= 1;

        if (State.RemainingGuesses <= 0) {
            State.Status = HangmanStatus.Lose;
        }
    }

   public delegate void EventHandler<T>(HangmanGame game, T t);

   public event EventHandler<HangmanState> StateChanged;
}
