test_input = open('inputs/test.txt').readlines()
main_input = open('inputs/main.txt').readlines()

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
    horizontal: bool

    def __init__(self, p1: Point, p2: Point):
        self.horizontal = p1.y == p2.y

        # This should be given by the test cases
        if not self.horizontal:
            assert(p1.x == p2.x)

        # if horizontal, then put the smaller x to p1
        if self.horizontal:
            self.p1 = p1 if p1.x < p2.x else p2
            self.p2 = p1 if p1.x > p2.x else p2
        # If vertical, then put the smaller y to p1
        else:
            self.p1 = p1 if p1.y < p2.y else p2
            self.p2 = p1 if p1.y > p2.y else p2

    def __str__(self):
        return f"{"H" if self.horizontal else "V"} {self.p1} - {self.p2})"

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

def intersects(l1: Line, l2: Line) -> bool:
    """
    Checks whether l1 is intersected by l2
    """
    # Don't compare parallel lines / tangents
    if (l1.horizontal == l2.horizontal):
        print("OH NO!", l1, l2)
    assert(l1.horizontal != l2.horizontal)
    # Compare a -- with a |
    if l1.horizontal and not l2.horizontal:
        y = l1.p1.y # should be the same as l1.p2.y
        x = l2.p1.x # should be the same as l2.p2.x

        within_y = l2.p1.y <= y and l2.p2.y >= y
        within_x = l1.p1.x < x and l1.p2.x > x
        return within_y and within_x
    elif not l1.horizontal and l2.horizontal:
        x = l1.p1.x # should be the same as l1.p2.x
        y = l2.p1.y # should be the same as l2.p2.y

        within_x = l2.p1.x <= x and l2.p2.x >= x
        within_y = l1.p1.y < y and l1.p2.y > y
        return within_y and within_x

    # This should never be reached
    print("OH NO!")
    return True

def is_rect_legal(rect: Rect, lines: list[Line]) -> bool:
    p1, p2 = rect.p1, rect.p2
    x1, x2 = min(p1.x, p2.x), max(p1.x, p2.x)
    y1, y2 = min(p1.y, p2.y), max(p1.y, p2.y)
    left = Line(Point(x1, y1), Point(x1, y2))
    right = Line(Point(x2, y1), Point(x2, y2))
    top = Line(Point(x1, y1), Point(x2, y1))
    bottom = Line(Point(x1, y1), Point(x2, y1))

    # Force set because the points can be the same
    left.horizontal = False
    right.horizontal = False
    top.horizontal = True
    bottom.horizontal = True

    for i in range(len(lines)):
        line = lines[i]
        if line.horizontal:
            if intersects(left, line) or intersects(right, line):
                return False
        else:
            if intersects(top, line) or intersects(bottom, line):
                return False

    print(f"{rect} is legal")
    return True
    
def solve(points: list[Point]):
    rectangles: list[Rect] = []
    lines: list[Line] = []
    for i in range(len(points)):
        next = i + 1 if i < len(points) - 1 else 0
        lines.append(Line(points[i], points[next]))

        for j in range(i + 1, len(points)):
            rectangles.append(Rect(points[i], points[j]))

    rectangles.sort(key=lambda rectangle: rectangle.area, reverse=True)

    max_area = 0
    for i in range(len(rectangles)):
        if is_rect_legal(rectangles[i], lines):
            max_area = rectangles[i].area
            break

    print("Part One is:", rectangles[0].area)
    print("Part Two is:", max_area)


test_points = get_points(test_input)
main_points = get_points(main_input)
solve(test_points)
solve(main_points)


