using System.Text.RegularExpressions;

namespace Days;

public partial class Day3
{
  public struct Number
  {
    public int value;
    public int row;
    public int start;
    public int length;
  }

  public class Game
  {
    public string[] lines;
    public List<Number> numbers = [];

    public Game(string input) {
      lines = input.Split('\n');
      for (int i = 0; i < lines.Length; ++i)
      {
        MatchCollection matches = NumberRegex().Matches(lines[i]);
        foreach (Match match in matches)
        {
          string matchString = match.ToString();
          Number number = new()
          {
            value = int.Parse(matchString),
            row = i,
            start = match.Index,
            length = matchString.Length
          };
          numbers.Add(number);
        }
      }
    }
  }

  [GeneratedRegex(@"(\d+)")]
  private static partial Regex NumberRegex();

  public static void Solve()
  {
    string input = Utils.ReadFile("inputs/03");
    Utils.PrintDay(3);
    Game game = new(input);
    Utils.PrintPartOne(PartOne(game));
  }

  private static bool IsValid(string[] grid, int row, int col) {
    // checks that are out of bounds don't matter
    if (row < 0 || col < 0 || row >= grid.Length || col >= grid[0].Length) return false;
    char c = grid[row][col];
    // don't bother if the neighbour is a number or period, we already know
    if (char.IsNumber(c)) return false;
    if (c == '.') return false;
    return true;
  }

  private static bool IsValid(string[] grid, Number number) {
    for (int col = number.start; col < number.start + number.length; ++col) {
      // if ()
    }
    return true;
  }

  public static int PartOne(Game game)
  {
    int sum = 0;
    foreach (Number number in game.numbers) {
      if (IsValid(number, game.lines)) {
        sum += number.value;
      }
    }
    return sum;
  }
}
