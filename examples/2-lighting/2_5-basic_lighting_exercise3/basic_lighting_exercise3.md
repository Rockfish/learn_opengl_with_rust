# Basic Lighting Exercise 3

## Exercise 3
Implement Gouraud shading instead of Phong shading. If you did things right the lighting should look a 
bit off as you can see at: learnopengl.com/img/lighting/basic_lighting_exercise3.png (especially the 
specular highlights) with the cube object. Try to reason why it looks so weird.

### So what do we see?
You can see (for yourself or in the provided image) the clear distinction of the two triangles at the front of the
cube. This 'stripe' is visible because of fragment interpolation. From the example image we can see that the top-right
vertex of the cube's front face is lit with specular highlights. Since the top-right vertex of the bottom-right triangle is
lit and the other 2 vertices of the triangle are not, the bright values interpolates to the other 2 vertices. The same
happens for the upper-left triangle. Since the intermediate fragment colors are not directly from the light source
but are the result of interpolation, the lighting is incorrect at the intermediate fragments and the top-left and
bottom-right triangle collide in their brightness resulting in a visible stripe between both triangles.

This effect will become more apparent when using more complicated shapes.

### Solution

Vertex Shader

    #version 330 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec3 aNormal;
    
    out vec3 LightingColor; // resulting color from lighting calculations
    
    uniform vec3 lightPos;
    uniform vec3 viewPos;
    uniform vec3 lightColor;
    
    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 projection;
    
    void main()
    {
        gl_Position = projection * view * model * vec4(aPos, 1.0);
    
        // gouraud shading
        // ------------------------
        vec3 Position = vec3(model * vec4(aPos, 1.0));
        vec3 Normal = mat3(transpose(inverse(model))) * aNormal;
        
        // ambient
        float ambientStrength = 0.1;
        vec3 ambient = ambientStrength * lightColor;
  	    
        // diffuse 
        vec3 norm = normalize(Normal);
        vec3 lightDir = normalize(lightPos - Position);
        float diff = max(dot(norm, lightDir), 0.0);
        vec3 diffuse = diff * lightColor;
        
        // specular
        float specularStrength = 1.0; // this is set higher to better show the effect of Gouraud shading 
        vec3 viewDir = normalize(viewPos - Position);
        vec3 reflectDir = reflect(-lightDir, norm);  
        float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
        vec3 specular = specularStrength * spec * lightColor;      
    
        LightingColor = ambient + diffuse + specular;
    }

Fragment Shader

    #version 330 core
    out vec4 FragColor;
    
    in vec3 FragPos;
    in vec3 Normal;
    in vec3 LightPos;   // extra in variable, since we need the light position in view space we calculate this in the vertex shader
    
    uniform vec3 lightColor;
    uniform vec3 objectColor;
    
    void main()
    {
        // ambient
        float ambientStrength = 0.1;
        vec3 ambient = ambientStrength * lightColor;
    
         // diffuse 
        vec3 norm = normalize(Normal);
        vec3 lightDir = normalize(LightPos - FragPos);
        float diff = max(dot(norm, lightDir), 0.0);
        vec3 diffuse = diff * lightColor;
        
        // specular
        float specularStrength = 0.5;
        vec3 viewDir = normalize(-FragPos); // the viewer is always at (0,0,0) in view-space, so viewDir is (0,0,0) - Position => -Position
        vec3 reflectDir = reflect(-lightDir, norm);  
        float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
        vec3 specular = specularStrength * spec * lightColor; 
        
        vec3 result = (ambient + diffuse + specular) * objectColor;
        FragColor = vec4(result, 1.0);
    }