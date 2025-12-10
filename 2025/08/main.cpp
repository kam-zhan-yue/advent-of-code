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

struct Scene {
  Shader lightShader;
  unsigned int lightVAO;
  unsigned int colourVBO;
};

// OpenGL Stuff
void framebuffer_size_callback(GLFWwindow* window, int width, int height);
void processInput(GLFWwindow *window, float &deltaTime);
void mouseCallback(GLFWwindow *window, double xPos, double yPos);
void scrollCallback(GLFWwindow *window, double xPos, double yPos);

// Rendering Stuff
Scene generateScene();
void updateScene(Scene scene);
void renderScene(Scene scene);

const float INTERVAL = 0.2;
const glm::vec3 INACTIVE = glm::vec3(0.2);

// Global Variables
bool firstMouse = false;
bool running = false;
bool spacePressed = false;
float tick = 0.0f;
float deltaTime = 0.0f;
float lastFrame = 0.0f;
float lastX = 400.0f;
float lastY = 300.0f;
Camera camera(glm::vec3(0.0f, 0.0f, 3.0f));
Solver solver;
default_random_engine generator;
uniform_real_distribution<double> distribution;

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
  GLFWwindow* window = glfwCreateWindow(800, 600, "LearnOpenGL", NULL, NULL);
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
    glClearColor(0.1f, 0.1f, 0.1f, 1.0f);
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
    updateScene(scene);
    renderScene(scene);

    // End Render Loop
    glfwSwapBuffers(window);
    glfwPollEvents();
  }
  return 0;
}

const float COMPRESSOR = 10000;

unsigned int generateLightVAO() {
  float vertices[] = {
    // Points           // Colours, Radius, Maybe
    0.0f, 0.0f, 0.0f
  };

  unsigned int VAO, VBO;
  // Generate
  glGenVertexArrays(1, &VAO);
  glGenBuffers(1, &VBO);

  // Bind
  glBindVertexArray(VAO);
  glBindBuffer(GL_ARRAY_BUFFER, VBO);

  // Data
  glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);
  glEnableVertexAttribArray(0);
  glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 3 * sizeof(float), (void*)0);

  vector<glm::vec3> positions;
  vector<Point> points = solver.graph.points;
  for (int i=0; i<points.size(); ++i) {
    Point point = points[i];
    glm::vec3 position;
    position.x = (float)point.x;
    position.y = (float)point.y;
    position.z = (float)point.z;
    // Compress so that they are not too far away
    positions.push_back(position / COMPRESSOR);
  }

  // Position Data (Instanced Array)
  unsigned int positionVBO;
  glGenBuffers(1, &positionVBO);
  glBindBuffer(GL_ARRAY_BUFFER, positionVBO);
  glBufferData(GL_ARRAY_BUFFER, sizeof(glm::vec3) * points.size(), &positions[0], GL_STATIC_DRAW);
  glEnableVertexAttribArray(1);
  glVertexAttribPointer(1, 3, GL_FLOAT, GL_FALSE, 3 * sizeof(float), (void*)0);
  glVertexAttribDivisor(1, 1);

  // Unbind
  glBindVertexArray(0);
  glBindBuffer(GL_ARRAY_BUFFER, 0);

  return VAO;
}

unsigned int generateColourVBO(unsigned int lightVAO) {
  glBindVertexArray(lightVAO);
  vector<glm::vec3> colours;
  for (int i=0; i<solver.graph.points.size(); ++i) {
    colours.push_back(INACTIVE);
  }

  // Colour Data (Instanced Array) - it seems like we would have to merge them otherwise
  unsigned int colourVBO;
  glGenBuffers(1, &colourVBO);
  glBindBuffer(GL_ARRAY_BUFFER, colourVBO);
  glBufferData(GL_ARRAY_BUFFER, sizeof(glm::vec3) *  solver.graph.points.size(), colours.data(), GL_DYNAMIC_DRAW);
  glEnableVertexAttribArray(2);
  glVertexAttribPointer(2, 3, GL_FLOAT, GL_FALSE, 3 * sizeof(float), (void*)0);
  glVertexAttribDivisor(2, 1);
  glBindVertexArray(0);
  glBindBuffer(GL_ARRAY_BUFFER, 0);

  return colourVBO;
}

Scene generateScene() {
  Shader lightShader = Shader(
    "shaders/light-vertex.glsl",
    "shaders/light-fragment.glsl"
  );

  // Generate Vertex Arrays
  unsigned int lightVAO = generateLightVAO();
  
  unsigned int colourVBO = generateColourVBO(lightVAO);

  return {
    lightShader,
    lightVAO,
    colourVBO,
  };
}

void updateScene(Scene scene) {
  if (!running) {
    return;
  }

  // Iterate the solver on each tick of the program
  tick -= deltaTime;
  if (tick < 0) {
    solver.tick();

    vector<glm::vec3> colours;
    for (int i=0; i<solver.graph.points.size(); ++i) {
      int circuit_num = solver.graph.get_circuit_num(i);
      if (circuit_num == -1) {
        colours.push_back(INACTIVE);
      } else {
        float r = distribution(generator);
        float g = distribution(generator);
        float b = distribution(generator);
        glm::vec3 colour = glm::vec3(r, g, b);
        colours.push_back(colour);
      }
    }
    // Update the colours accordingly
    glBindVertexArray(scene.lightVAO);
    glBindBuffer(GL_ARRAY_BUFFER, scene.colourVBO);
    glBufferSubData(GL_ARRAY_BUFFER, 0, sizeof(glm::vec3) * colours.size(), colours.data());
    glBindVertexArray(0);
    glBindBuffer(GL_ARRAY_BUFFER, 0);
  }
}

void renderScene(Scene scene) {
  scene.lightShader.use();
  scene.lightShader.setVec3("cameraPos", camera.cameraPos);
  scene.lightShader.setMat4("view", camera.getLookAt());
  scene.lightShader.setMat4("projection", camera.getPerspective());
  scene.lightShader.setMat4("model", glm::mat4(1.0));
  glBindVertexArray(scene.lightVAO);
  glDrawArraysInstanced(GL_POINTS, 0, 1, solver.graph.points.size());
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
