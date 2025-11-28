#pragma once

#include "Font.h"
#include "Themes.h"
#include "imgui.h"
#include "backends/imgui_impl_android.h"  // ❌ SỬA: XÓA "ImGui/"
#include "backends/imgui_impl_opengl3.h"  // ❌ SỬA: XÓA "ImGui/"
#include "imgui_internal.h"
#include "ImguiPP.h"

#include "Icon.h"
#include "Iconcpp.h"
#include "imgui_demo.cpp"
#include "Touch.h"
#include "Image/ImageTexture.h"


bool CircularButton(const char* label_text, float button_size = 64.0f, ImU32 button_color = IM_COL32(255, 255, 255, 255), ImVec4 TextColor = ImVec4(1.00f, 1.00f, 1.00f, 1.00f)) {
    ImGuiStyle &style = ImGui::GetStyle();
    ImVec4 original_button_color = style.Colors[ImGuiCol_Button];
    float original_frame_rounding = style.FrameRounding;
    ImVec4 original_text_color = style.Colors[ImGuiCol_Text];
    style.Colors[ImGuiCol_Button] = ImVec4(ImGui::ColorConvertU32ToFloat4(button_color));
    style.FrameRounding = button_size * 0.5f;
    style.Colors[ImGuiCol_Text] = TextColor;
    bool button_pressed = ImGui::Button(label_text, ImVec2(button_size, button_size));
    style.Colors[ImGuiCol_Button] = original_button_color;
    style.FrameRounding = original_frame_rounding;
    style.Colors[ImGuiCol_Text] = original_text_color;

    return button_pressed;
}

float convertIntToFloat(int input) {
    return input / 2.0f + 0.5f;
}
void drawRhombus(ImDrawList* drawList, ImVec2 center, float size) {
    ImVec2 points[4] = {
            ImVec2(center.x, center.y - size),
            ImVec2(center.x + size, center.y),
            ImVec2(center.x, center.y + size),
            ImVec2(center.x - size, center.y)
    };

    drawList->AddConvexPolyFilled(points, 4, IM_COL32(255, 255, 0, 255));  // Fill the rhombus with yellow color
    drawList->AddPolyline(points, 4, IM_COL32(255, 0, 0, 255), true, 2.0f);  // Draw the border with red color
}

void ImGui_RunningText(const char* text, float speed, ImU32 textColor = 0)
{
    static float offset = 0.0f;
    static float textWidth = 0.0f;
    static bool initialized = false;

    if (!initialized)
    {
        ImGui::PushFont(ImGui::GetIO().Fonts->Fonts[0]);
        textWidth = ImGui::CalcTextSize(text).x;
        ImGui::PopFont();
        initialized = true;
    }

    offset -= speed;

    if (offset < -textWidth)
    {
        offset = ImGui::GetWindowWidth();
    }

    ImGui::SetCursorPosX(offset);

    ImVec4 color;
    if (textColor == 0)
    {
        color = ImGui::ColorConvertU32ToFloat4(ImGui::GetColorU32(ImGuiCol_Text));
    }
    else
    {
        color = ImGui::ColorConvertU32ToFloat4(textColor);
    }

    ImGui::TextColored(color, "%s", text);
}

static void MetricsHelpMarker(const char* desc)
{
    ImGui::TextDisabled("(?)");
    if (ImGui::IsItemHovered(ImGuiHoveredFlags_DelayShort) && ImGui::BeginTooltip())
    {
        ImGui::PushTextWrapPos(ImGui::GetFontSize() * 35.0f);
        ImGui::TextUnformatted(desc);
        ImGui::PopTextWrapPos();
        ImGui::EndTooltip();
    }
}

const char* GetCurrentTime(bool date = false) {
    std::time_t currentTime = std::time(nullptr);
    std::tm* timeInfo = std::localtime(&currentTime);

    std::ostringstream oss;
    if (date) {
        oss << std::put_time(timeInfo, "%Y-%m-%d %H:%M:%S");
    } else {
        oss << std::put_time(timeInfo, "%H:%M:%S");
    }
    std::string timeString = oss.str();
    return strdup(timeString.c_str());
}

void DrawImage(ImDrawList *draw, int x, int y, int w, int h, ImTextureID Texture)
{
    draw->AddImage(Texture, ImVec2(x, y), ImVec2(x + w, y + h));
}

long GetEpochTime()
{
    auto duration = std::chrono::system_clock::now().time_since_epoch();
    return std::chrono::duration_cast<std::chrono::milliseconds>(duration).count();
}

ImColor GetRainbowColor(float speed, float num1)
{
    speed = 0.002f * speed;
    long now = GetEpochTime();
    float hue = (now * (int)(num1 / speed)) * speed;
    return ImColor::HSV(hue, 1.0f, 1.0f);
}

ImColor SwitchColor(float speed,int num1)
{
    speed = 0.002f * speed;
    long now = GetEpochTime();
    float hue = (now % (int)(num1 / speed)) * speed;
    return ImColor::HSV(hue,1.0f, 1.0f);//57,98,100(yellow) 0,98,100(red)
}


