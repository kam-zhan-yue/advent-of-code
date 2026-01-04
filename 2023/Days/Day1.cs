namespace Days;
using System.Text.RegularExpressions;

public partial class Day1
{
  [GeneratedRegex(@"(\d)")]
  private static partial Regex PartOneRegex();

  public static void Solve()
  {
    Utils.PrintDay(1);
    string input = Utils.ReadFile("inputs/01");
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
      int firstDigit = int.Parse(matches[0].ToString());
      int lastDigit = int.Parse(matches[^1].ToString());
      sum += firstDigit * 10 + lastDigit;
    }
    return sum;
  }

  public static int PartTwo(string input)
  {
    return 0;
  }
}
