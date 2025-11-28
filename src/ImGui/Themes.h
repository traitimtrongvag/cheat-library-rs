#pragma once
#include "imgui.h"
#include "gradient.h"

void DrawImGuiStyle(){
    ImGuiStyle& style = ImGui::GetStyle();

    style.WindowRounding = 5.3f;
    style.FrameRounding = 2.3f;
    style.ScrollbarRounding = 12.0f;
    style.FrameRounding = 2.0f;
    style.FrameBorderSize = 0.6f;
    style.ChildRounding = 8.0f;

    style.Colors[ImGuiCol_Text] = ImVec4(0.95f, 0.96f, 0.98f, 1.0f);
    style.Colors[ImGuiCol_TextDisabled] = ImVec4(0.50f, 0.50f, 0.50f, 1.0f);
    style.Colors[ImGuiCol_WindowBg] = ImVec4(0.10f, 0.10f, 0.13f, 1.0f);
    style.Colors[ImGuiCol_ChildBg] = ImVec4(0.12f, 0.12f, 0.15f, 1.0f);
    style.Colors[ImGuiCol_PopupBg] = ImVec4(0.10f, 0.10f, 0.13f, 0.98f);
    style.Colors[ImGuiCol_Border] = ImVec4(0.20f, 0.22f, 0.27f, 1.0f);
    style.Colors[ImGuiCol_BorderShadow] = ImVec4(0.0f, 0.0f, 0.0f, 0.0f);
    style.Colors[ImGuiCol_FrameBg] = ImVec4(0.16f, 0.17f, 0.21f, 1.0f);
    style.Colors[ImGuiCol_FrameBgHovered] = ImVec4(0.20f, 0.22f, 0.27f, 1.0f);
    style.Colors[ImGuiCol_FrameBgActive] = ImVec4(0.26f, 0.28f, 0.34f, 1.0f);
    style.Colors[ImGuiCol_TitleBg] = ImVec4(0.08f, 0.08f, 0.10f, 1.0f);
    style.Colors[ImGuiCol_TitleBgActive] = ImVec4(0.12f, 0.12f, 0.15f, 1.0f);
    style.Colors[ImGuiCol_TitleBgCollapsed] = ImVec4(0.08f, 0.08f, 0.10f, 1.0f);
    style.Colors[ImGuiCol_MenuBarBg] = ImVec4(0.12f, 0.12f, 0.15f, 1.0f);
    style.Colors[ImGuiCol_ScrollbarBg] = ImVec4(0.10f, 0.10f, 0.13f, 1.0f);
    style.Colors[ImGuiCol_ScrollbarGrab] = ImVec4(0.26f, 0.28f, 0.34f, 1.0f);
    style.Colors[ImGuiCol_ScrollbarGrabHovered] = ImVec4(0.32f, 0.34f, 0.40f, 1.0f);
    style.Colors[ImGuiCol_ScrollbarGrabActive] = ImVec4(0.38f, 0.40f, 0.46f, 1.0f);
    style.Colors[ImGuiCol_CheckMark] = ImVec4(0.26f, 0.82f, 0.96f, 1.0f);
    style.Colors[ImGuiCol_SliderGrab] = ImVec4(0.26f, 0.82f, 0.96f, 0.70f);
    style.Colors[ImGuiCol_SliderGrabActive] = ImVec4(0.26f, 0.82f, 0.96f, 1.0f);
    style.Colors[ImGuiCol_Button] = ImVec4(0.16f, 0.17f, 0.21f, 1.0f);
    style.Colors[ImGuiCol_ButtonHovered] = ImVec4(0.26f, 0.82f, 0.96f, 0.20f);
    style.Colors[ImGuiCol_ButtonActive] = ImVec4(0.26f, 0.82f, 0.96f, 0.35f);
    style.Colors[ImGuiCol_Header] = ImVec4(0.16f, 0.17f, 0.21f, 1.0f);
    style.Colors[ImGuiCol_HeaderHovered] = ImVec4(0.26f, 0.82f, 0.96f, 0.25f);
    style.Colors[ImGuiCol_HeaderActive] = ImVec4(0.26f, 0.82f, 0.96f, 0.40f);
    style.Colors[ImGuiCol_Separator] = ImVec4(0.20f, 0.22f, 0.27f, 1.0f);
    style.Colors[ImGuiCol_SeparatorHovered] = ImVec4(0.26f, 0.82f, 0.96f, 0.60f);
    style.Colors[ImGuiCol_SeparatorActive] = ImVec4(0.26f, 0.82f, 0.96f, 1.0f);
    style.Colors[ImGuiCol_ResizeGrip] = ImVec4(0.26f, 0.82f, 0.96f, 0.25f);
    style.Colors[ImGuiCol_ResizeGripHovered] = ImVec4(0.26f, 0.82f, 0.96f, 0.60f);
    style.Colors[ImGuiCol_ResizeGripActive] = ImVec4(0.26f, 0.82f, 0.96f, 1.0f);
    style.Colors[ImGuiCol_Tab] = ImVec4(0.12f, 0.12f, 0.15f, 1.0f);
    style.Colors[ImGuiCol_TabHovered] = ImVec4(0.26f, 0.82f, 0.96f, 0.30f);
    style.Colors[ImGuiCol_TabActive] = ImVec4(0.20f, 0.22f, 0.27f, 1.0f);
    style.Colors[ImGuiCol_TabUnfocused] = ImVec4(0.10f, 0.10f, 0.13f, 1.0f);
    style.Colors[ImGuiCol_TabUnfocusedActive] = ImVec4(0.16f, 0.17f, 0.21f, 1.0f);
    style.Colors[ImGuiCol_PlotLines] = ImVec4(0.26f, 0.82f, 0.96f, 1.0f);
    style.Colors[ImGuiCol_PlotLinesHovered] = ImVec4(0.40f, 0.90f, 1.0f, 1.0f);
    style.Colors[ImGuiCol_PlotHistogram] = ImVec4(0.26f, 0.82f, 0.96f, 1.0f);
    style.Colors[ImGuiCol_PlotHistogramHovered] = ImVec4(0.40f, 0.90f, 1.0f, 1.0f);
    style.Colors[ImGuiCol_TableHeaderBg] = ImVec4(0.12f, 0.12f, 0.15f, 1.0f);
    style.Colors[ImGuiCol_TableBorderStrong] = ImVec4(0.20f, 0.22f, 0.27f, 1.0f);
    style.Colors[ImGuiCol_TableBorderLight] = ImVec4(0.16f, 0.17f, 0.21f, 1.0f);
    style.Colors[ImGuiCol_TableRowBg] = ImVec4(0.12f, 0.12f, 0.15f, 1.0f);
    style.Colors[ImGuiCol_TableRowBgAlt] = ImVec4(0.14f, 0.14f, 0.17f, 1.0f);
    style.Colors[ImGuiCol_TextSelectedBg] = ImVec4(0.26f, 0.82f, 0.96f, 0.35f);
    style.Colors[ImGuiCol_DragDropTarget] = ImVec4(0.26f, 0.82f, 0.96f, 0.90f);
    style.Colors[ImGuiCol_NavHighlight] = ImVec4(0.26f, 0.82f, 0.96f, 1.0f);
    style.Colors[ImGuiCol_NavWindowingHighlight] = ImVec4(0.26f, 0.82f, 0.96f, 1.0f);
    style.Colors[ImGuiCol_NavWindowingDimBg] = ImVec4(0.0f, 0.0f, 0.0f, 0.60f);
    style.Colors[ImGuiCol_ModalWindowDimBg] = ImVec4(0.0f, 0.0f, 0.0f, 0.60f);
};