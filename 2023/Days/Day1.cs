namespace Days;

using System.Text.RegularExpressions;

public partial class Day1
{
  [GeneratedRegex(@"(\d)")]
  private static partial Regex PartOneRegex();

  [GeneratedRegex(@"(\d|one|two|three|four|five|six|seven|eight|nine)")]
  private static partial Regex PartTwoRegex();

  public static void Solve()
  {
    string input = Utils.ReadFile("inputs/01");
    Utils.PrintDay(1);
    Utils.PrintPartOne(PartOne(input));
    Utils.PrintPartTwo(PartTwo(input));
  }

  public static int PartOne(string input)
  {
    Regex regex = PartOneRegex();
    int sum = 0;
    string[] lines = input.Split('\n');
    foreach (string line in lines)
    {
      MatchCollection matches = regex.Matches(line);
      if (matches.Count == 0) continue;
      int firstDigit = int.Parse(matches[0].ToString());
      int lastDigit = int.Parse(matches[^1].ToString());
      sum += firstDigit * 10 + lastDigit;
    }
    return sum;
  }

  public static int PartTwo(string input)
  {
    static string Duplicate(string value) {
      string final = value;
      final = final.Replace("one", "oneone");
      final = final.Replace("two", "twotwo");
      final = final.Replace("three", "threethree");
      final = final.Replace("four", "fourfour");
      final = final.Replace("five", "fivefive");
      final = final.Replace("six", "sixsix");
      final = final.Replace("seven", "sevenseven");
      final = final.Replace("eight", "eighteight");
      final = final.Replace("nine", "ninenine");
      return final;
    }

    static int ParseNumber(string value)
    {
      return value switch
      {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => int.Parse(value),
      };
    }
    Regex regex = PartTwoRegex();
    int sum = 0;
    string[] lines = input.Split('\n');
    foreach (string line in lines)
    {
      string duplicated = Duplicate(line);
      MatchCollection matches = regex.Matches(duplicated);
      if (matches.Count == 0) continue;
      int firstDigit = ParseNumber(matches[0].ToString());
      int lastDigit = ParseNumber(matches[^1].ToString());
      sum += firstDigit * 10 + lastDigit;
    }
    return sum;
  }
}
