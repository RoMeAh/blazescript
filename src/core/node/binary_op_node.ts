import { ParseResult } from "../parser/parse_result.ts";
import { Token } from "../token.ts";
import { NumberNode } from "./number_nodes.ts";

export class BinOpNode {
  constructor(
    public leftNode: ParseResult | NumberNode | BinOpNode | null,
    public opToken: Token,
    public rightNode: ParseResult | NumberNode | BinOpNode | null,
  ) {
  }

  public represent(): string {
    if(!this.leftNode) return "()";
    if(!this.rightNode) return "()";

    let toBeappended = ""
    if(this.leftNode instanceof ParseResult) {
      toBeappended += `(${this.leftNode.node?.represent()}, ${this.opToken.represent()}, `
    } else {
      toBeappended += `(${this.leftNode.represent()}, ${this.opToken.represent()}, `
    }

    if(this.rightNode instanceof ParseResult) {
      toBeappended += `${this.rightNode.node?.represent()})`
    } else {
      toBeappended += `${this.rightNode.represent()})`
    }

    return toBeappended;
  }
}
