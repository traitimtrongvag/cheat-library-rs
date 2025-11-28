#ifndef IMGUI_WRAPPER_H
#define IMGUI_WRAPPER_H

#include <stddef.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

void ImGui_CreateContext();
bool ImGui_InitOpenGL3(const char* glsl_version);
void ImGui_AndroidNewFrame(int width, int height);
void ImGui_OpenGL3NewFrame();
void ImGui_NewFrame();
void ImGui_Render();
void ImGui_EndFrame();
void ImGui_RenderDrawData();

void ImGui_Text(const char* text);
bool ImGui_Button(const char* label, float width, float height);
bool ImGui_Checkbox(const char* label, bool* v);
bool ImGui_SliderInt(const char* label, int* v, int v_min, int v_max);
bool ImGui_SliderFloat(const char* label, float* v, float v_min, float v_max);
void ImGui_Separator();
void ImGui_SameLine();
void ImGui_Spacing();

bool ImGui_Begin(const char* name, bool* p_open, int flags);
void ImGui_End();
bool ImGui_BeginChild(const char* str_id, float width, float height, bool border, int flags);
void ImGui_EndChild();

void ImGui_OpenPopup(const char* str_id);
bool ImGui_BeginPopupModal(const char* name, bool* p_open, int flags);
void ImGui_EndPopup();

void ImGui_PushStyleColor(int idx, float r, float g, float b, float a);
void ImGui_PopStyleColor(int count);
void ImGui_PushStyleVar(int idx, float val);
void ImGui_PopStyleVar(int count);
void ImGui_PushItemWidth(float width);
void ImGui_PopItemWidth();

bool ImGui_InputText(const char* label, char* buf, size_t buf_size, int flags);

void ImGui_Columns(int count, const char* id, bool border);
void ImGui_NextColumn();
void ImGui_SetColumnOffset(int column_index, float offset_x);

bool ImGui_BeginTable(const char* str_id, int column, int flags);
void ImGui_EndTable();
void ImGui_TableNextColumn();

void* ImGui_GetIO();
float ImGui_GetFramerate();

void DrawImGuiStyle_Wrapper();

#ifdef __cplusplus
}
#endif

#endif // IMGUI_WRAPPER_H