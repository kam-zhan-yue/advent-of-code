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
        self.p1 = p1
        self.p2 = p2
        self.horizontal = p1.x == p2.x

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

def is_legal(points: list[Point], a: int, b: int) -> bool:
    p1, p2 = points[a], points[b]
    x1, x2 = min(p1.x, p2.x), max(p1.x, p2.x)
    y1, y2 = min(p1.y, p2.y), max(p1.y, p2.y)
    for i in range(len(points)):
        if i == a or i == b: 
            continue
        p = points[i]
        within_x = p.x > x1 and p.x < x2
        within_y = p.y > y1 and p.y < y2
        if within_x and within_y:
            print(f"{p} is within {p1} and {p2}")
            return False

    return True

def is_rect_legal(rect: Rect, points: list[Point]) -> bool:
    p1, p2 = rect.p1, rect.p2
    x1, x2 = min(p1.x, p2.x), max(p1.x, p2.x)
    y1, y2 = min(p1.y, p2.y), max(p1.y, p2.y)
    for i in range(len(points)):
        p = points[i]
        if p == p1 or p == p2:
            continue
        within_x = p.x > x1 and p.x < x2
        within_y = p.y > y1 and p.y < y2
        if within_x and within_y:
            print(f"{p} is within {p1} and {p2}")
            return False

    print(f"Found for {p1} and {p2}")
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
        if is_rect_legal(rectangles[i], points):
            max_area = rectangles[i].area
            break

    print("Part One is:", rectangles[0].area)
    print("Part Two is:", max_area)


test_points = get_points(test_input)
solve(test_points)

# main_points = get_points(main_input)
# part_one(main_points)
# part_two(main_points)
