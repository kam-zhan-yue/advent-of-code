#include <iostream>
#include <glad/glad.h>
#include <GLFW/glfw3.h>
#include <glm/glm.hpp>
#include <solution.h>
#include <shader.h>
#include <camera.h>
#include <random>
#include <chrono>

using namespace std;

struct Shaders {
  Shader light;
  Shader line;
};

struct Lights {
  unsigned int VAO;
  unsigned int positionBuffer;
  unsigned int colourBuffer;
};

struct Lines {
  unsigned int VAO;
  unsigned int positionBuffer;
};

struct Scene {
  Shaders shaders;
  Lights lights;
  Lines lines;
};

// OpenGL Stuff
void framebuffer_size_callback(GLFWwindow* window, int width, int height);
void processInput(GLFWwindow *window, float &deltaTime);
void mouseCallback(GLFWwindow *window, double xPos, double yPos);
void scrollCallback(GLFWwindow *window, double xPos, double yPos);
glm::vec3 point_to_vec3(Point point);

// Rendering Stuff
Scene generateScene();
void updateScene(Scene scene);
void renderScene(Scene scene);

const float INTERVAL = 0.01;
const float COMPRESSOR = 10000;
const glm::vec3 OFFSET = glm::vec3(-5, -5, -5);
const glm::vec3 INACTIVE = glm::vec3(0.2);

// Global Variables
bool firstMouse = false;
bool running = false;
bool spacePressed = false;
bool jPressed = false;
bool tickRequested = false;
float tick = 0.0f;
float deltaTime = 0.0f;
float lastFrame = 0.0f;
float lastX = 400.0f;
float lastY = 300.0f;
Camera camera(glm::vec3(0.0f, 0.0f, 3.0f));
Solver solver;
default_random_engine generator;
uniform_real_distribution<double> distribution;
map<int, glm::vec3> circuitMap;

// Galaxy Pallete
vector<glm::vec3> palette = {
    {0.9f, 0.9f, 1.0f},  // star white
    {0.6f, 0.7f, 1.0f},  // blue star
    {0.9f, 0.6f, 1.0f},  // nebula purple
    {1.0f, 0.5f, 0.8f},  // magenta cloud
    {0.6f, 0.4f, 0.9f},  // deep violet
    {0.9f, 0.7f, 0.5f},  // dusty glow
};

GLFWwindow *init() {
  // Init GLFW and set the context variables
  glfwInit();
  glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
  glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
  glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
  #ifdef __APPLE__
  glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GL_TRUE);
  #endif

  // Create a window object
  GLFWwindow* window = glfwCreateWindow(800, 600, "Day 8: Playground", NULL, NULL);
  if (window == NULL) {
    cout << "Failed to create GLFW window" << endl;
    glfwTerminate();
    throw runtime_error("Failed to initialize GLFW");
  }
  glfwMakeContextCurrent(window);

  // Initialise GLAD
  if (!gladLoadGLLoader((GLADloadproc)glfwGetProcAddress)) {
    cout << "Failed to initialise GLAD" << endl;
    throw runtime_error("Failed to initialize GLAD");
  }

  // Tell OpenGL the size of the rendering window so that OpenGL knows how we want to display the data and coordinates
  glViewport(0, 0, 800, 600);

  // Register the frame buffer size callback when the user resizes the window
  glfwSetFramebufferSizeCallback(window, framebuffer_size_callback);

  // Hide cursor and register cursor callback
  glfwSetInputMode(window, GLFW_CURSOR, GLFW_CURSOR_DISABLED);
  glfwSetCursorPosCallback(window, mouseCallback);
  glfwSetScrollCallback(window, scrollCallback);

  glEnable(GL_DEPTH_TEST);
  glEnable(GL_PROGRAM_POINT_SIZE);
  glEnable(GL_LINE_SMOOTH);
  glEnable(GL_BLEND);
  glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

  return window;
}

int main() {
  GLFWwindow *window = init();

  // Randonmess
  unsigned seed = std::chrono::system_clock::now().time_since_epoch().count();
  default_random_engine generator(seed);
  uniform_real_distribution<double> distribution(0.0, 1.0);

  Scene scene = generateScene();

  while (!glfwWindowShouldClose(window)) {
    // Init Render Loop
    float currentFrame = glfwGetTime();
    deltaTime = currentFrame - lastFrame;
    lastFrame = currentFrame;
    processInput(window, deltaTime);

    // Render Stuff
    glClearColor(0.0f, 0.0f, 0.0f, 1.0f);
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
    updateScene(scene);
    renderScene(scene);

    // End Render Loop
    glfwSwapBuffers(window);
    glfwPollEvents();
  }
  return 0;
}

Lights generateLights() {
  unsigned int VAO;
  glGenVertexArrays(1, &VAO);
  glBindVertexArray(VAO);

  vector<glm::vec3> positions;
  vector<glm::vec3> colours;
  vector<Point> points = solver.graph.points;
  for (int i=0; i<points.size(); ++i) {
    positions.push_back(point_to_vec3(points[i]));
    colours.push_back(INACTIVE);
  }

  // Position Data (Instanced Array)
  unsigned int positionBuffer;
  glGenBuffers(1, &positionBuffer);
  glBindBuffer(GL_ARRAY_BUFFER, positionBuffer);
  glBufferData(GL_ARRAY_BUFFER, sizeof(glm::vec3) * points.size(), &positions[0], GL_STATIC_DRAW);
  glEnableVertexAttribArray(0);
  glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 3 * sizeof(float), (void*)0);
  glVertexAttribDivisor(0, 1);

  // Colour Data (Instanced Array) - it seems like we would have to merge them otherwise
  unsigned int colourBuffer;
  glGenBuffers(1, &colourBuffer);
  glBindBuffer(GL_ARRAY_BUFFER, colourBuffer);
  glBufferData(GL_ARRAY_BUFFER, sizeof(glm::vec3) *  solver.graph.points.size(), colours.data(), GL_DYNAMIC_DRAW);
  glEnableVertexAttribArray(1);
  glVertexAttribPointer(1, 3, GL_FLOAT, GL_FALSE, 3 * sizeof(float), (void*)0);
  glVertexAttribDivisor(1, 1);
  glBindVertexArray(0);
  glBindBuffer(GL_ARRAY_BUFFER, 0);

  // Unbind
  glBindVertexArray(0);
  glBindBuffer(GL_ARRAY_BUFFER, 0);

  return { VAO, positionBuffer, colourBuffer };
}

Lines generateLines() {
  unsigned int VAO;
  glGenVertexArrays(1, &VAO);
  glBindVertexArray(VAO);

  vector<glm::vec3> points;
  for (int i=0; i<solver.graph.points.size() * 2; ++i) {
    points.push_back(glm::vec3(0.0));
  }
  
  unsigned int positionBuffer;
  glGenBuffers(1, &positionBuffer);
  glBindBuffer(GL_ARRAY_BUFFER, positionBuffer);
  glBufferData(GL_ARRAY_BUFFER, sizeof(glm::vec3) * points.size(), points.data(), GL_DYNAMIC_DRAW);
  glEnableVertexAttribArray(0);
  glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 3 * sizeof(float), (void*)0);

  glBindVertexArray(0);
  glBindBuffer(GL_ARRAY_BUFFER, 0);
  return { VAO, positionBuffer };
}

Shaders generateShaders() {
  Shader light = Shader("shaders/light-vertex.glsl", "shaders/light-fragment.glsl");
  Shader line = Shader("shaders/line-vertex.glsl", "shaders/line-fragment.glsl");
  return { light, line };
}

Scene generateScene() {
  return {
    .shaders = generateShaders(),
    .lights = generateLights(),
    .lines = generateLines(),
  };
}

glm::vec3 point_to_vec3(Point point) {
  glm::vec3 position;
  position.x = (float)point.x;
  position.y = (float)point.y;
  position.z = (float)point.z;
  position /= COMPRESSOR;
  position.x += OFFSET.x;
  position.y += OFFSET.y;
  position.z += OFFSET.z;
  return position;
}

void updateScene(Scene scene) {
  if (!running && !tickRequested) {
    return;
  }
  tickRequested = false;

  // Iterate the solver on each tick of the program
  tick -= deltaTime;
  if (tick < 0) {
    int ticked = solver.tick();
    while (ticked == 0) {
      ticked = solver.tick();
    }
    vector<glm::vec3> colours;
    for (int i=0; i<solver.graph.points.size(); ++i) {
      int circuitNum = solver.graph.get_circuit_num(i);
      if (circuitNum == -1) {
        colours.push_back(INACTIVE);
      } else if (circuitMap.count(circuitNum)){
        glm::vec3 colour = circuitMap[circuitNum];
        colours.push_back(colour);
      } else {
        int i = rand() % palette.size();
        int j = rand() % palette.size();
        float t = distribution(generator);
        glm::vec3 colour = glm::mix(palette[i], palette[j], t);
        colours.push_back(colour);
        circuitMap.insert({ circuitNum, colour });
      }
    }

    // Update the colours accordingly
    glBindVertexArray(scene.lights.VAO);
    glBindBuffer(GL_ARRAY_BUFFER, scene.lights.colourBuffer);
    glBufferSubData(GL_ARRAY_BUFFER, 0, sizeof(glm::vec3) * colours.size(), colours.data());

    // Update the lines
    vector<glm::vec3> connections;
    for (int i=0; i<solver.graph.connections.size(); ++i) {
      connections.push_back(point_to_vec3(solver.graph.connections[i].a));
      connections.push_back(point_to_vec3(solver.graph.connections[i].b));
    }
    glBindVertexArray(scene.lines.VAO);
    glBindBuffer(GL_ARRAY_BUFFER, scene.lines.positionBuffer);
    glBufferSubData(GL_ARRAY_BUFFER, 0, sizeof(glm::vec3) * connections.size(), connections.data());

    // Unbind
    glBindVertexArray(0);
    glBindBuffer(GL_ARRAY_BUFFER, 0);

    // Turn off ticking
    if (solver.graph.is_complete())  {
      running = false;
    }
  }
}

void renderScene(Scene scene) {
  // Render Lights
  Shader light = scene.shaders.light;
  light.use();
  light.setMat4("view", camera.getLookAt());
  light.setMat4("projection", camera.getPerspective());
  light.setMat4("model", glm::mat4(1.0));
  glBindVertexArray(scene.lights.VAO);
  glDrawArraysInstanced(GL_POINTS, 0, 1, solver.graph.points.size());

  // Render Lines
  Shader line = scene.shaders.line;
  line.use();
  line.setMat4("view", camera.getLookAt());
  line.setMat4("projection", camera.getPerspective());
  line.setMat4("model", glm::mat4(1.0));
  glBindVertexArray(scene.lines.VAO);
  glDrawArrays(GL_LINES, 0, solver.graph.connections.size() * 2);
}

void framebuffer_size_callback(GLFWwindow* window, int width, int height) {
  glViewport(0, 0, width, height);
}

void processInput(GLFWwindow *window, float &deltaTime) {
  if (glfwGetKey(window, GLFW_KEY_ESCAPE) == GLFW_PRESS) {
    glfwSetWindowShouldClose(window, true);
  }
  if (glfwGetKey(window, GLFW_KEY_SPACE) == GLFW_PRESS) {
    if (!spacePressed) {
      running = !running;
      spacePressed = true;
    }
  } else {
    spacePressed = false;
  }
  if (glfwGetKey(window, GLFW_KEY_J) == GLFW_PRESS && !tickRequested) {
    if (!jPressed) {
      tickRequested = true;
      jPressed = true;
    }
  } else {
    jPressed = false;
  }
  camera.process(window, deltaTime);
}

void mouseCallback(GLFWwindow *window, double xPos, double yPos) {
  if (firstMouse) {
    lastX = xPos;
    lastY = yPos;
    firstMouse = false;
  }

  float xOffset = xPos - lastX;
  float yOffset = lastY - yPos;
  lastX = xPos;
  lastY = yPos;

  camera.processMouse(xOffset, yOffset);
}

void scrollCallback(GLFWwindow *window, double xOffset, double yOffset) {
  camera.processScroll(xOffset, yOffset);
}
