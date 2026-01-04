using System.Text.RegularExpressions;

namespace Days;

public readonly partial struct Game
{
  public Game(string line)
  {
    Sets = [];
    MatchCollection matches = GameRegex().Matches(line);
    foreach (Match match in matches)
    {
      if (match.Groups.Count < 3) continue;
      ID = int.Parse(match.Groups[1].ToString());
      string[] sets = match.Groups[2].ToString().Split(";");
      for (int i = 0; i < sets.Length; ++i)
      {
        Sets.Add(new Set(sets[i]));
      }
    }
  }

  public int ID { get; init; }
  public List<Set> Sets { get; init; }

  [GeneratedRegex(@"Game (\d+): (.*)")]
  private static partial Regex GameRegex();
}

public readonly partial struct Set
{
  public Set(string set)
  {
    MatchCollection matches = SetRegex().Matches(set.Trim());

    foreach (Match match in matches)
    {
      if (match.Groups.Count < 3) continue;
      int count = int.Parse(match.Groups[2].ToString());
      string colour = match.Groups[3].ToString();
      switch (colour)
      {
        case "blue":
          Blue = count;
          break;
        case "red":
          Red = count;
          break;
        case "green":
          Green = count;
          break;
      }
    }
  }

  public int Blue { get; init; }
  public int Red { get; init; }
  public int Green { get; init; }

  [GeneratedRegex(@"((\d+) (blue|red|green))")]
  private static partial Regex SetRegex();
}

public partial class Day2
{
  public static void Solve()
  {
    string input = Utils.ReadFile("inputs/02");
    Game[] games = ParseInput(input);
    Utils.PrintDay(2);
    Utils.PrintPartOne(PartOne(games));
    Utils.PrintPartTwo(PartTwo(games));
  }

  public static Game[] ParseInput(string input)
  {
    string[] lines = input.Split('\n');
    Game[] games = new Game[lines.Length];

    for (int i = 0; i < lines.Length; ++i)
    {
      games[i] = new Game(lines[i]);
    }
    return games;
  }

  private static bool IsGameLegal(Game game, Set bag) {
    foreach (Set set in game.Sets) {
      if (set.Blue > bag.Blue) return false;
      if (set.Red > bag.Red) return false;
      if (set.Green > bag.Green) return false;
    }
    return true;
  }

  public static int PartOne(Game[] games)
  {
    Set bag = new()
    {
      Red = 12,
      Green = 13,
      Blue = 14,
    };

    int sum = 0;
    foreach (Game game in games) {
      if (IsGameLegal(game, bag)) {
        sum += game.ID;
      }
    }
    return sum;
  }

  private static int GetMinimumCubes(Game game) {
    int red, green, blue;
    red = green = blue = 0;
    foreach (Set set in game.Sets) {
      if (set.Red > red) red = set.Red;
      if (set.Blue > blue) blue = set.Blue;
      if (set.Green > green) green = set.Green;
    }
    return red * green * blue;
  }

  public static int PartTwo(Game[] games)
  {
    int sum = 0;
    foreach (Game game in games) {
      sum += GetMinimumCubes(game);
    }
    return sum;
  }
}
