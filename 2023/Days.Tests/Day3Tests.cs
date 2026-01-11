namespace Days.Tests;

public class Day3Tests
{
const string INPUT = 
@"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

  [Test]
  public void TestPartOne()
  {
    Day3.Game game = Day3.ParseInput(INPUT);
    Assert.That(Day3.PartOne(game), Is.EqualTo(8));
  }
}
