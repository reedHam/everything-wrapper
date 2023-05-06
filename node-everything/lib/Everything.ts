import {
    everythingLib,
    toWideString,
    fromWideString,
    EVERYTHING_ERRORS,
    EVERYTHING_SORT,
} from "./ffi";
import ref from "ref-napi";

export class Everything {
    private parseStringPointer(pointer: Buffer) {
        if (pointer.isNull()) {
            let lastError = this.getLastError();
            throw new Error(lastError);
        } else {
            let buffer = ref.reinterpretUntilZeros(pointer, 2);

            return fromWideString(buffer);
        }
    }

    isDbLoaded() {
        return everythingLib.Everything_IsDBLoaded();
    }

    async waitDbLoaded() {
        const timeout = setTimeout(() => {
            throw new Error("Everything DB load timeout");
        }, 10000);

        while (!this.isDbLoaded()) {
            await new Promise((resolve) => setTimeout(resolve, 100));
        }
        clearTimeout(timeout);
    }

    version() {
        return everythingLib.Everything_GetMajorVersion();
    }

    getLastError() {
        let error_id = everythingLib.Everything_GetLastError();
        if (error_id in EVERYTHING_ERRORS) {
            return EVERYTHING_ERRORS[
                error_id as keyof typeof EVERYTHING_ERRORS
            ];
        } else {
            return "UNKNOWN Error";
        }
    }

    setSearch(search: string) {
        let buffer = toWideString(search);

        everythingLib.Everything_SetSearchW(buffer as any);
    }

    getSearch() {
        let pointer = everythingLib.Everything_GetSearchW();
        return this.parseStringPointer(pointer);
    }

    setSort(sort: number) {
        everythingLib.Everything_SetSort(sort);
    }

    getSort() {
        return everythingLib.Everything_GetSort();
    }

    setMaxResults(max: number) {
        everythingLib.Everything_SetMax(max);
    }

    getMaxResults() {
        return everythingLib.Everything_GetMax();
    }

    setOffset(offset: number) {
        everythingLib.Everything_SetOffset(offset);
    }

    getOffset() {
        return everythingLib.Everything_GetOffset();
    }

    getNumResults() {
        return everythingLib.Everything_GetNumResults();
    }

    query() {
        return everythingLib.Everything_QueryW(true);
    }

    getResultPath(index: number) {
        const path_length = everythingLib.Everything_GetResultFullPathNameW(
            index,
            ref.NULL as any,
            0
        );

        // Allocate buffer with an extra character for the NULL terminator.
        let buffer = Buffer.alloc((path_length + 1) * 2);

        everythingLib.Everything_GetResultFullPathNameW(
            index,
            buffer as any,
            path_length + 1
        );

        return this.parseStringPointer(buffer);
    }

    *pathIter() {
        let numResults = this.getNumResults();
        console.log("numResults", numResults);
        for (let i = 0; i < numResults; i++) {
            yield this.getResultPath(i);
        }
    }

    getResultFileName(index: number) {
        let pointer = everythingLib.Everything_GetResultFileNameW(index);
        return this.parseStringPointer(pointer);
    }

    *fileNameIter() {
        let numResults = this.getNumResults();
        for (let i = 0; i < numResults; i++) {
            yield this.getResultFileName(i);
        }
    }

    cleanup() {
        everythingLib.Everything_CleanUp();
    }
}
