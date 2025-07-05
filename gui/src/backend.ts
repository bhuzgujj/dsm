import {invoke} from "@tauri-apps/api/core";
import {DsmSets, MergedSet} from "./backend.types";

export async function listRawDatasets() {
    return invoke<DsmSets[]>("list_raw", { })
}

export async function listMergedDatasets() {
    return invoke<MergedSet[]>("list_merged", { })
}