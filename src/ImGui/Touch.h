bool setup = false;
#define HOOK(ret, func, ...) \
    ret (*orig##func)(__VA_ARGS__); \
    ret my##func(__VA_ARGS__)

HOOK(void, Input, void *thiz, void *ex_ab, void *ex_ac) {
    origInput(thiz, ex_ab, ex_ac);
    ImGui_ImplAndroid_HandleInputEvent((AInputEvent *)thiz);
    return;
}

bool ShowMenu;

jmethodID MotionEvent_getX;
jmethodID MotionEvent_getY;
jmethodID MotionEvent_getAction;
jmethodID KeyEvent_getUnicodeChar;
jmethodID KeyEvent_getMetaState;
jmethodID KeyEvent_getAction;
jmethodID KeyEvent_getKeyCode;

int (*o_inject_event)(JNIEnv *env, jobject thiz, jobject inputEvent);
int hook_input(JNIEnv *env, jobject __this, jobject input_event) {
    ImGuiIO &io = ImGui::GetIO();

    jclass motionEventClass = env->FindClass(OBFUSCATE("android/view/MotionEvent"));

    if (env->IsInstanceOf(input_event, motionEventClass)) {
        jmethodID getActionMethod = env->GetMethodID(motionEventClass, OBFUSCATE("getAction"), OBFUSCATE("()I"));
        jint getAction = env->CallIntMethod(input_event, getActionMethod);

        jmethodID getXMethod = env->GetMethodID(motionEventClass, OBFUSCATE("getX"), OBFUSCATE("()F"));
        jfloat getX = env->CallFloatMethod(input_event, getXMethod);

        jmethodID getYMethod = env->GetMethodID(motionEventClass, OBFUSCATE("getY"), OBFUSCATE("()F"));
        jfloat getY = env->CallFloatMethod(input_event, getYMethod);

        jmethodID getPointerCountMethod = env->GetMethodID(motionEventClass, OBFUSCATE("getPointerCount"), OBFUSCATE("()I"));
        jint getPointerCount = env->CallIntMethod(input_event, getPointerCountMethod);

        switch(getAction) {

            case 0:
                io.MouseDown[0] = true;
                break;
            case 1:
                io.MouseDown[0] = false;
                break;
            case 2:
                if (getPointerCount > 1) {
                    io.MouseDown[0] = false;
                } else {
                    io.MouseWheel = 0;
                }
                break;
        }
        io.MousePos = ImVec2(getX, getY);
    }

    jclass KeyEventClass = env->FindClass(OBFUSCATE("android/view/KeyEvent"));
    if (env->IsInstanceOf(input_event, KeyEventClass)) {
        jmethodID getActionMethod = env->GetMethodID(KeyEventClass, OBFUSCATE("getAction"), OBFUSCATE("()I"));
        if (env->CallIntMethod(input_event, getActionMethod) == 0) {
            jmethodID getKeyCodeMethod = env->GetMethodID(KeyEventClass, OBFUSCATE("getKeyCode"), OBFUSCATE("()I"));
            jmethodID getUnicodeCharMethod = env->GetMethodID(KeyEventClass, OBFUSCATE("getUnicodeChar"), OBFUSCATE("(I)I"));
            jmethodID getMetaStateMethod = env->GetMethodID(KeyEventClass, OBFUSCATE("getMetaState"), OBFUSCATE("()I"));

            jint keyCode = env->CallIntMethod(input_event, getKeyCodeMethod);
            switch (keyCode)
            {
                case 19:
                    io.KeysDown[io.KeyMap[ImGuiKey_UpArrow]] = true;
                    break;
                case 20:
                    io.KeysDown[io.KeyMap[ImGuiKey_DownArrow]] = true;
                    break;
                case 21:
                    io.KeysDown[io.KeyMap[ImGuiKey_LeftArrow]] = true;
                    break;
                case 22:
                    io.KeysDown[io.KeyMap[ImGuiKey_RightArrow]] = true;
                    break;
                case 66:
                    io.KeysDown[io.KeyMap[ImGuiKey_Enter]] = true;
                    break;
                case 67:
                    io.KeysDown[io.KeyMap[ImGuiKey_Backspace]] = true;;
                    break;
                case 111:
                    io.KeysDown[io.KeyMap[ImGuiKey_Escape]] = true;
                    break;
                case 112:
                    io.KeysDown[io.KeyMap[ImGuiKey_Delete]] = true;
                    break;
                case 122:
                    io.KeysDown[io.KeyMap[ImGuiKey_Home]] = true;
                    break;
                case 123:
                    io.KeysDown[io.KeyMap[ImGuiKey_End]] = true;
                    break;
                default:
                    io.AddInputCharacter(env->CallIntMethod(input_event, getUnicodeCharMethod, env->CallIntMethod(input_event, getMetaStateMethod)));
                    break;
            }
        }
    }
        jclass MotionEventCls = env->FindClass("android/view/MotionEvent");
        jclass KeyEventCls = env->FindClass("android/view/KeyEvent");
        if (env->IsInstanceOf(input_event, KeyEventCls))
        {
            if (!KeyEvent_getAction)
                KeyEvent_getAction = env->GetMethodID(KeyEventCls, "getAction", "()I");
            if (env->CallIntMethod(input_event, KeyEvent_getAction) == 0)
            {
                if (!KeyEvent_getKeyCode)
                    KeyEvent_getKeyCode = env->GetMethodID(KeyEventCls, "getKeyCode", "()I");
                if (!KeyEvent_getUnicodeChar)
                    KeyEvent_getUnicodeChar = env->GetMethodID(KeyEventCls, "getUnicodeChar", "(I)I");
                if (!KeyEvent_getMetaState)
                    KeyEvent_getMetaState = env->GetMethodID(KeyEventCls, "getMetaState", "()I");
                ImGuiIO &io = ImGui::GetIO();
                int KeyCode = env->CallIntMethod(input_event, KeyEvent_getKeyCode);
                switch (KeyCode)
                {
                    case 24:
                        ShowMenu = true;
                        break;
                    case 25:
                        ShowMenu = false;
                        break;
                    default:
                        io.AddInputCharacter(env->CallIntMethod(input_event, KeyEvent_getUnicodeChar, env->CallIntMethod(input_event, KeyEvent_getMetaState)));
                        break;
                }
            }
        }
    return o_inject_event(env, __this, input_event);
}

int32_t (*orig_ANativeWindow_getWidth)(ANativeWindow* window);
int32_t _ANativeWindow_getWidth(ANativeWindow* window) {
    screenWidth = orig_ANativeWindow_getWidth(window);
    return orig_ANativeWindow_getWidth(window);
}

int32_t (*orig_ANativeWindow_getHeight)(ANativeWindow* window);
int32_t _ANativeWindow_getHeight(ANativeWindow* window) {
    screenHeight = orig_ANativeWindow_getHeight(window);
    return orig_ANativeWindow_getHeight(window);
}

void (*SetResolution)(int width, int height, bool fullscreen);
void (*lon)(int cac, int cac1, bool conl);
int Width, Height;