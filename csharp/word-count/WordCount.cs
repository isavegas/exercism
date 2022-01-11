using System;
using System.Collections.Generic;
using System.Linq;
using System.Text.RegularExpressions;

public static class WordCount {
    public static string FixQuotes(string word) {
        while ((word.StartsWith('\'') && word.EndsWith('\'') || word.StartsWith('\"') && word.EndsWith('"'))) {
            word = word.Substring(1, word.Length - 2);
        }

        return word;
    }

    public static IDictionary<string, int> CountWords(string phrase) {
        Dictionary<string, int> map = new Dictionary<string, int>();

        string[] words = new Regex("[\\w']+").Matches(phrase).Select(match => FixQuotes(match.Groups[0].Value.ToLower())).ToArray();
        foreach (string word in words) {
            map[word] = map.GetValueOrDefault(word, 0) + 1;
        }

        return map;
    }
}
