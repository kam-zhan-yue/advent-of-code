namespace Days.Tests;

public class Tests
{

  [Test]
  public void TestPartOne()
  {
    string INPUT = @"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    Assert.That(Day1.PartOne(INPUT), Is.EqualTo(142));
  }

  [Test]
  public void TestPartTwo()
  {
    Assert.Pass();
    string INPUT = @"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
    Assert.That(Day1.PartTwo(INPUT), Is.EqualTo(281));
  }
}
