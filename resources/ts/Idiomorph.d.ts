declare module "idiomorph" {
  class Idiomorph {
    static morph(
      existingNode: HTMLElement,
      newContent: string,
      options?: {
        head?: { style: "merge" | "append" | "morph" | "none" };
        morphStyle?: "innerHTML" | "outerHTML";
      },
    ): void;
  }
}
