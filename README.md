Movement on WASD and arrows

It is impossible in glium 0.18.1 to implement shadow mapping in proper way

reason: In current glutin (0.18.1), there is no implementation of `GL_TEXTURE_COMPARE_MODE` and `GL_TEXTURE_COMPARE_FUNC` OpenGL features.
