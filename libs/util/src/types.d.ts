import {components} from "./api_v1";

export type Item = components["schemas"]["Item"];
export type Form = components["schemas"]["Form"];
export type FormResponse = components["schemas"]["FormResponse"]
export type Info = components["schemas"]["Info"]

export type FormItemType = ("question_text" | "text" | "page_break" | "question_radio_button" | "question_check_box");
export type SaveStatus = "saving" | "saved" | "unsaved";
