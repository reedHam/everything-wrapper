import ffi from "ffi-napi";
import ref from "ref-napi";
import iconv from "iconv-lite";

const DWORD = ref.types.uint32;
const BOOL = ref.types.bool;
const LPCWSTR = ref.refType(ref.types.uint16);
const LPWSTR = ref.refType(ref.types.uint16);

// Define the FFI library based on the Everything SDK's DLL
export const everythingLib = ffi.Library(
    String.raw`C:\github\everything-wrapper\Everything-SDK\dll\Everything64.dll`,
    {
        Everything_SetSearchW: ["void", [LPCWSTR]],
        Everything_GetSearchW: [LPWSTR, []],
        Everything_SetSort: ["void", [DWORD]],
        Everything_GetSort: [DWORD, []],
        Everything_IsFastSort: [BOOL, [DWORD]],
        Everything_IsFileResult: [BOOL, [DWORD]],
        Everything_IsFolderResult: [BOOL, [DWORD]],
        Everything_IsVolumeResult: [BOOL, [DWORD]],
        Everything_GetNumResults: [DWORD, []],
        Everything_GetTotResults: [DWORD, []],
        Everything_SetMax: ["void", [DWORD]],
        Everything_GetMax: [DWORD, []],
        Everything_SetOffset: ["void", [DWORD]],
        Everything_GetOffset: [DWORD, []],
        Everything_QueryW: [BOOL, [BOOL]],
        Everything_Reset: ["void", []],
        Everything_GetLastError: [DWORD, []],
        Everything_GetResultFullPathNameW: [DWORD, [DWORD, LPWSTR, DWORD]],
        Everything_GetResultFileNameW: [LPWSTR, [DWORD]],
        Everything_IsDBLoaded: [BOOL, []],
        Everything_CleanUp: ["void", []],
        Everything_GetMajorVersion: [DWORD, []],
    }
);

export const EVERYTHING_ERRORS = {
    0: "OK",
    1: "MEMORY",
    2: "IPC",
    3: "REGISTERCLASSEX",
    4: "CREATEWINDOW",
    5: "CREATETHREAD",
    6: "INVALIDINDEX",
    7: "INVALIDCALL",
    8: "INVALIDREQUEST",
    9: "INVALIDPARAMETER",
} as const;

export const EVERYTHING_SORT = {
    NAME_ASCENDING: 1,
    NAME_DESCENDING: 2,
    PATH_ASCENDING: 3,
    PATH_DESCENDING: 4,
    SIZE_ASCENDING: 5,
    SIZE_DESCENDING: 6,
    TYPE_ASCENDING: 7,
    TYPE_DESCENDING: 8,
    DATE_CREATED_ASCENDING: 9,
    DATE_CREATED_DESCENDING: 10,
    DATE_MODIFIED_ASCENDING: 11,
    DATE_MODIFIED_DESCENDING: 12,
    ATTRIBUTES_ASCENDING: 13,
    ATTRIBUTES_DESCENDING: 14,
    FILE_LIST_FILENAME_ASCENDING: 15,
    FILE_LIST_FILENAME_DESCENDING: 16,
    RUN_COUNT_ASCENDING: 17,
    RUN_COUNT_DESCENDING: 18,
    DATE_RECENTLY_CHANGED_ASCENDING: 19,
    DATE_RECENTLY_CHANGED_DESCENDING: 20,
    DATE_ACCESSED_ASCENDING: 21,
    DATE_ACCESSED_DESCENDING: 22,
    DATE_RUN_ASCENDING: 23,
    DATE_RUN_DESCENDING: 24,
} as const;

// Utility functions to convert between wide strings and UTF-8 strings
export function toWideString(str: string): Buffer {
    const buffer = iconv.encode(str + "\0", "utf-16le");
    return buffer;
}

export function fromWideString(buffer: Buffer): string {
    return iconv.decode(buffer, "utf-16le");
}
