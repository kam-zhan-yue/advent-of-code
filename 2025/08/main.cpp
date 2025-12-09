#include <iostream>
#include <glad/glad.h>
#include <GLFW/glfw3.h>
#include <glm/glm.hpp>
#include <solution.h>
#include <shader.h>
#include <camera.h>

using namespace std;

struct Scene {
  Shader lightShader;
  unsigned int lightVAO;
};

// OpenGL Stuff
void framebuffer_size_callback(GLFWwindow* window, int width, int height);
void processInput(GLFWwindow *window, float &deltaTime);
void mouseCallback(GLFWwindow *window, double xPos, double yPos);
void scrollCallback(GLFWwindow *window, double xPos, double yPos);

// Rendering Stuff
Scene generateScene();
void renderScene(Scene scene);

// Global Variables
bool firstMouse = false;
float deltaTime = 0.0f;
float lastFrame = 0.0f;
float lastX = 400.0f;
float lastY = 300.0f;
Camera camera(glm::vec3(0.0f, 0.0f, 3.0f));

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
  int connections = get_connections();
  vector<Point> points = get_points();
  solve(points, connections);

  GLFWwindow *window = init();

  // Build Shaders
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
    renderScene(scene);

    // End Render Loop
    glfwSwapBuffers(window);
    glfwPollEvents();
  }
  return 0;
}

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

  // TODO: Replace with positions
  glm::vec3 positions[100];
  int index = 0;
  float offset = 0.1f;
  for (int y=-10; y<10; y+=2) {
    for (int x=-10; x<10; x+= 2) {
      glm::vec3 translation;
      translation.x = (float)x / 10.0f + offset;
      translation.y = (float)y / 10.0f + offset;
      translation.z = 0;
      positions[index++] = translation;
    }
  }

  unsigned int instanceVBO;
  glGenBuffers(1, &instanceVBO);
  glBindBuffer(GL_ARRAY_BUFFER, instanceVBO);
  glBufferData(GL_ARRAY_BUFFER, sizeof(glm::vec3) * 100, &positions[0], GL_STATIC_DRAW);
  glEnableVertexAttribArray(1);
  glVertexAttribPointer(1, 3, GL_FLOAT, GL_FALSE, 3 * sizeof(float), (void*)0);
  glVertexAttribDivisor(1, 1);

  // Unbind
  glBindVertexArray(0);
  glBindBuffer(GL_ARRAY_BUFFER, 0);

  return VAO;
}

Scene generateScene() {
  Shader lightShader = Shader(
    "shaders/light-vertex.glsl",
    "shaders/light-fragment.glsl"
  );

  // Generate Vertex Arrays
  unsigned int lightVAO = generateLightVAO();

  return {
    lightShader,
    lightVAO
  };
}

void renderScene(Scene scene) {
  scene.lightShader.use();
  scene.lightShader.setVec3("cameraPos", camera.cameraPos);
  /*float length = glm::length(camera.cameraPos);*/
  /*cout << "Camera Pos is " << length << endl;*/
  scene.lightShader.setMat4("view", camera.getLookAt());
  scene.lightShader.setMat4("projection", camera.getPerspective());
  scene.lightShader.setMat4("model", glm::mat4(1.0));
  glBindVertexArray(scene.lightVAO);
  glDrawArraysInstanced(GL_POINTS, 0, 1, 100);

  /*glDrawArrays(GL_POINTS, 0, 1);*/
}

void framebuffer_size_callback(GLFWwindow* window, int width, int height) {
  glViewport(0, 0, width, height);
}

void processInput(GLFWwindow *window, float &deltaTime) {
  if (glfwGetKey(window, GLFW_KEY_ESCAPE) == GLFW_PRESS) {
    glfwSetWindowShouldClose(window, true);
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
