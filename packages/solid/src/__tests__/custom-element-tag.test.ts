import { describe, expect, it } from "bun:test"
import { createHostElement } from "../host.js"

// A crate that links `gpuix-native` registers elements such as `<my-chart>`.
// The Solid host used to throw for every tag it did not know, so the element
// could be registered in Rust and still never mount.
describe("custom element tags", () => {
  it("accepts a hyphenated tag registered from Rust", () => {
    expect(createHostElement("my-chart").type).toBe("my-chart")
  })

  it("still rejects a tag without a hyphen", () => {
    expect(() => createHostElement("mychart")).toThrow(
      "Unsupported GPUIX element <mychart>"
    )
  })
})
