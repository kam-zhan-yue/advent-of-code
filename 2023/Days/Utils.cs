namespace Days {
  class Utils {
    public static string ReadFile(string filename) {
      StreamReader sr = new(filename);
      return sr.ReadToEnd();
    }

    public static void PrintDay(int day) {
      Console.WriteLine($"==========DAY {day}==========");
    }

    public static void PrintPartOne(int result) {
      Console.WriteLine($"Part One: {result}");
    }

    public static void PrintPartTwo(int result) {
      Console.WriteLine($"Part Two: {result}");
    }
  }
}
