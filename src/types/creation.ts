export enum Operation {
    "add",
    "sub",
    "mul",
    "div",
    "set",
}

export type PriorityBundleEffect = {
    id: number;
    bundle_id: number;
    target_domain: "attribute" | "skill" | "resource";
    target_name: string; // Key path
    operation: Operation;
    value: number;
    created_at: string;
    updated_at: string;
};

export type MethodSpec = {
    title: string;
    effects: PriorityBundleEffect[];
};
