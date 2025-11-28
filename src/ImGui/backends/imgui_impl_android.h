


#pragma once
#include "../imgui.h"      // IMGUI_IMPL_API
#ifndef IMGUI_DISABLE

struct ANativeWindow;
struct AInputEvent;

IMGUI_IMPL_API bool     ImGui_ImplAndroid_Init(ANativeWindow* window);
IMGUI_IMPL_API int32_t  ImGui_ImplAndroid_HandleInputEvent(AInputEvent* input_event);
IMGUI_IMPL_API void     ImGui_ImplAndroid_Shutdown();
IMGUI_IMPL_API void     ImGui_ImplAndroid_NewFrame(int width, int height);

#endif // #ifndef IMGUI_DISABLE
