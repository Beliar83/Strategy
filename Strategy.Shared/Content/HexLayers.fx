#if OPENGL
	#define SV_POSITION POSITION
	#define VS_SHADERMODEL vs_3_0
	#define PS_SHADERMODEL ps_3_0
#else
	#define VS_SHADERMODEL vs_4_0_level_9_1
	#define PS_SHADERMODEL ps_4_0_level_9_1
#endif

matrix WorldViewProjection;
float3 Offset;

float4 HighlightColor;
bool HighlightActive;

bool AttackableActive;
float4 AttackableColor;

bool MoveableActive;
float4 MoveableColor;

struct VertexShaderInput
{
	float4 Position : POSITION0;
};

struct VertexShaderOutput
{
	float4 Position : SV_POSITION;
};

VertexShaderOutput MainVS(in VertexShaderInput input)
{
	VertexShaderOutput output = (VertexShaderOutput)0;

	float4 position = input.Position;
	position.xyz += Offset;    
    output.Position = mul(position, WorldViewProjection);
	
	return output;
}

float4 MainPS(VertexShaderOutput input) : COLOR
{
    float4 color = {1, 1, 1, 1};
    
    float factor = 1.0;
    
    if (MoveableActive) {
        color = lerp(color, MoveableColor, factor);
        factor = 0.5;
    }
    
    if (AttackableActive) {
        color = lerp(color, AttackableColor, factor);
        factor = 0.5;
    }

    if (HighlightActive) {
        color = lerp(color, HighlightColor, factor);
        factor = 0.5;
    }

	return color;
}

technique BasicColorDrawing
{
	pass P0
	{
		VertexShader = compile VS_SHADERMODEL MainVS();
		PixelShader = compile PS_SHADERMODEL MainPS();
	}
};