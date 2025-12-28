main_input = open('../inputs/09').readlines()

class Point:
    x: int
    y: int

    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    def __str__(self):
        return f"({self.x}, {self.y})"

class Line:
    p1: Point
    p2: Point

    def __init__(self, p1: Point, p2: Point):
        self.p1 = Point(min(p1.x, p2.x), min(p1.y, p2.y))
        self.p2 = Point(max(p1.x, p2.x), max(p1.y, p2.y))

    def intersects(self, rect: "Rect"):
        x, x2 = sorted([rect.p1.x, rect.p2.x])
        y, y2 = sorted([rect.p1.y, rect.p2.y])
        intersects_x = self.p1.x < x2 and self.p2.x > x
        intersects_y = self.p1.y < y2 and self.p2.y > y
        return intersects_x and intersects_y

    def __str__(self):
        return f"({self.p1}, {self.p2})"

class Rect:
    area: int
    p1: Point
    p2: Point

    def __init__(self, p1: Point, p2: Point):
        self.p1 = p1
        self.p2 = p2
        self.area = self.get_area(p1, p2)

    def get_area(self, a: Point, b: Point) -> int:
        x = abs(a.x - b.x) + 1
        y = abs(a.y - b.y) + 1
        return x * y

    def __str__(self):
        return f"Area {self.area} | {self.p1} - {self.p2}"

def get_points(lines: list[str]) -> list[Point]:
    points: list[Point] = []
    for line in lines:
        splits = line.split(',')
        points.append(Point(int(splits[0]), int(splits[1])))
    return points

def is_valid(rect: Rect, lines: list[Line]):
    # If a line intersects the rectangle, then it is invalid
    for line in lines:
        if line.intersects(rect):
            return False
    return True

def solve(points: list[Point]):
    rectangles: list[Rect] = []
    lines: list[Line] = []
    for i in range(len(points)):
        next = i + 1 if i + 1 < len(points) else 0
        lines.append(Line(points[i], points[next]))

        for j in range(i + 1, len(points)):
            rectangles.append(Rect(points[i], points[j]))

    rectangles.sort(key=lambda rectangle: rectangle.area, reverse=True)

    max_area = 0
    for i in range(len(rectangles)):
        if is_valid(rectangles[i], lines):
            max_area = rectangles[i].area
            break

    print("Part One is:", rectangles[0].area)
    print("Part Two is:", max_area)


main_points = get_points(main_input)
solve(main_points)
