#include "imgui.h"
#include "imgui_impl_android.h"
#include "imgui_impl_opengl3.h"
#include "imgui_wrapper.h"

extern "C" void ImGui_CreateContext() {
    ImGui::CreateContext();
}

extern "C" bool ImGui_InitOpenGL3(const char* glsl_version) {
    return ImGui_ImplOpenGL3_Init(glsl_version);
}

extern "C" void ImGui_AndroidNewFrame(int width, int height) {
    ImGui_ImplAndroid_NewFrame(width, height);
}

extern "C" void ImGui_OpenGL3NewFrame() {
    ImGui_ImplOpenGL3_NewFrame();
}

extern "C" void ImGui_NewFrame() {
    ImGui::NewFrame();
}

extern "C" void ImGui_Render() {
    ImGui::Render();
}

extern "C" void ImGui_EndFrame() {
    ImGui::EndFrame();
}

extern "C" void ImGui_RenderDrawData() {
    ImGui_ImplOpenGL3_RenderDrawData(ImGui::GetDrawData());
}

extern "C" void ImGui_Text(const char* text) {
    ImGui::Text("%s", text);
}

extern "C" bool ImGui_Button(const char* label, float width, float height) {
    return ImGui::Button(label, ImVec2(width, height));
}

extern "C" bool ImGui_Checkbox(const char* label, bool* v) {
    return ImGui::Checkbox(label, v);
}

extern "C" bool ImGui_SliderInt(const char* label, int* v, int v_min, int v_max) {
    return ImGui::SliderInt(label, v, v_min, v_max);
}

extern "C" bool ImGui_SliderFloat(const char* label, float* v, float v_min, float v_max) {
    return ImGui::SliderFloat(label, v, v_min, v_max);
}

extern "C" void ImGui_Separator() {
    ImGui::Separator();
}

extern "C" void ImGui_SameLine() {
    ImGui::SameLine();
}

extern "C" void ImGui_Spacing() {
    ImGui::Spacing();
}

extern "C" bool ImGui_Begin(const char* name, bool* p_open, int flags) {
    return ImGui::Begin(name, p_open, flags);
}

extern "C" void ImGui_End() {
    ImGui::End();
}

extern "C" bool ImGui_BeginChild(const char* str_id, float width, float height, bool border, int flags) {
    return ImGui::BeginChild(str_id, ImVec2(width, height), border, flags);
}

extern "C" void ImGui_EndChild() {
    ImGui::EndChild();
}

extern "C" void ImGui_OpenPopup(const char* str_id) {
    ImGui::OpenPopup(str_id);
}

extern "C" bool ImGui_BeginPopupModal(const char* name, bool* p_open, int flags) {
    return ImGui::BeginPopupModal(name, p_open, flags);
}

extern "C" void ImGui_EndPopup() {
    ImGui::EndPopup();
}

extern "C" void ImGui_PushStyleColor(int idx, float r, float g, float b, float a) {
    ImGui::PushStyleColor(idx, ImVec4(r, g, b, a));
}

extern "C" void ImGui_PopStyleColor(int count) {
    ImGui::PopStyleColor(count);
}

extern "C" void ImGui_PushStyleVar(int idx, float val) {
    ImGui::PushStyleVar(idx, val);
}

extern "C" void ImGui_PopStyleVar(int count) {
    ImGui::PopStyleVar(count);
}

extern "C" void ImGui_PushItemWidth(float width) {
    ImGui::PushItemWidth(width);
}

extern "C" void ImGui_PopItemWidth() {
    ImGui::PopItemWidth();
}

extern "C" bool ImGui_InputText(const char* label, char* buf, size_t buf_size, int flags) {
    return ImGui::InputText(label, buf, buf_size, flags);
}

extern "C" void ImGui_Columns(int count, const char* id, bool border) {
    ImGui::Columns(count, id, border);
}

extern "C" void ImGui_NextColumn() {
    ImGui::NextColumn();
}

extern "C" void ImGui_SetColumnOffset(int column_index, float offset_x) {
    ImGui::SetColumnOffset(column_index, offset_x);
}

extern "C" bool ImGui_BeginTable(const char* str_id, int column, int flags) {
    return ImGui::BeginTable(str_id, column, flags);
}

extern "C" void ImGui_EndTable() {
    ImGui::EndTable();
}

extern "C" void ImGui_TableNextColumn() {
    ImGui::TableNextColumn();
}

extern "C" void* ImGui_GetIO() {
    return (void*)&ImGui::GetIO();
}

extern "C" float ImGui_GetFramerate() {
    return ImGui::GetIO().Framerate;
}

extern "C" void DrawImGuiStyle_Wrapper() {
}