import { invoke } from "@tauri-apps/api/core";
import { ClassMapper, DsmSets, IncludedSet, MergedSet } from "./backend.types";

export function listRawDatasets() {
    return invoke<DsmSets[]>("list_raw", {})
}

export function listMergedDatasets() {
    return invoke<MergedSet[]>("list_merged", {})
}

export function storeRawSet(
    name: string,
    version: string,
    formats: string,
    path: string
) {
    return invoke<void>("store_raw_set", { name, version, formats, path })
}

export function getRawSet(
    name: string,
    version: string
) {
    return invoke<DsmSets | null>("get_raw_set", { name, version })
}

export function getMergedSet(
    name: string,
    version: string
) {
    return invoke<MergedSet | null>("get_merged_set", { name, version })
}

export function mergeNew(
    name: string,
    version: string,
    included: IncludedSet[],
    classMapping: ClassMapper,
) {
    return invoke<void>("merge_new", { name, version, included, classMapping })
}

export function generateMergedSet(
    name: string,
    version: string,
    formats: string,
    path: string,
) {
    return invoke<void>("generate_merged_set", { name, version, formats, path })
}
