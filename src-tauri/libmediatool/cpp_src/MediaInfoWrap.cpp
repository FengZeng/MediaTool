#include <cstddef>
#include <iostream>
#include "MediaInfo/MediaInfo.h"
#include "ZenLib/Ztring.h"

extern "C" const void* mi_Init() {
    MediaInfoLib::MediaInfo* mi = new MediaInfoLib::MediaInfo();
    mi->Option(ZenLib::Ztring("Input_Compressed"), ZenLib::Ztring(""));
    mi->Option(ZenLib::Ztring("Inform_Compress"), ZenLib::Ztring(""));
    return (const void*)mi;
}

extern "C" int mi_Open_Buffer_Init(const void* mi, uint64_t size, uint64_t offset) {
    MediaInfoLib::MediaInfo* pMI = (MediaInfoLib::MediaInfo*)mi;
    if (pMI == NULL) {
        return 0;
    } else {
        return pMI->Open_Buffer_Init(size, offset);
    }
}

extern "C" int mi_Open_Buffer_Continue(const void* mi, char* buffer, int size) {
    MediaInfoLib::MediaInfo* pMI = (MediaInfoLib::MediaInfo*)mi;
    if (pMI == NULL) {
        return 0;
    } else {
        return pMI->Open_Buffer_Continue((uint8_t*)buffer, (size_t)size);
    }
}

extern "C" int mi_Open_Buffer_Continue_GoTo_Get(const void* mi) {
    MediaInfoLib::MediaInfo* pMI = (MediaInfoLib::MediaInfo*)mi;
    if (pMI == NULL) {
        return 0;
    } else {
        return pMI->Open_Buffer_Continue_GoTo_Get();
    }
}

extern "C" int mi_Open_Buffer_Finalize(const void* mi) {
    MediaInfoLib::MediaInfo* pMI = (MediaInfoLib::MediaInfo*)mi;
    if (pMI == NULL) {
        return 0;
    } else {
        return pMI->Open_Buffer_Finalize();
    }
}

extern "C" const char* mi_Info(const void* mi) {
    MediaInfoLib::MediaInfo* pMI = (MediaInfoLib::MediaInfo*)mi;
    if (pMI == NULL) {
        return "";
    } else {
        ZenLib::Ztring str =pMI->Inform();
        //std::cout << str.To_Local() << std::endl;
        int strLen = str.To_Local().size() + 1;
        std::unique_ptr<char[]> buffer(new char[strLen + 1]);
        std::memcpy(buffer.get(), str.To_Local().c_str(), strLen + 1);
        buffer[strLen] = '\0';
        return buffer.release();
    }
}

extern "C" void mi_Close(const void* mi, const char* buffer) {
    MediaInfoLib::MediaInfo* pMI = (MediaInfoLib::MediaInfo*)mi;
    if (pMI != NULL) {
        delete pMI;
    }
    if (buffer != nullptr) {
        delete[] buffer;
    }
}

extern "C" const char*  mediainfo_open(const char* url) {
    ZenLib::Ztring filename(url);
    MediaInfoLib::MediaInfo MI;
    int ret = MI.Open(filename);
    ZenLib::Ztring str =MI.Inform();
    MI.Close();
    std::cout << "---zzz----mediainfo_open" << std::endl;
    std::cout << str.To_Local() << std::endl;
    std::cout << "---zzz----mediainfo_open" << std::endl;

    int strLen = str.To_Local().size() + 1;
    std::unique_ptr<char[]> buffer(new char[strLen + 1]);
    std::memcpy(buffer.get(), str.To_Local().c_str(), strLen + 1);
    buffer[strLen] = '\0';
    return buffer.release();
}

extern "C" void mediainfo_close(const char* buffer) {
    if (buffer != nullptr) {
        delete[] buffer;
    }
}
