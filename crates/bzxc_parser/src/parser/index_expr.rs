/*
 * Copyright 2020 to 2021 BlazifyOrg
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *    http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
*/

use super::Parser;
use crate::parse_result::ParseResult;
use bzxc_shared::{Error, Node, Tokens};

impl Parser {
    /*
     * Parse indexing of arrays
     */
    pub(crate) fn index_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let pos_start = self.current_token.pos_start.clone();
        let atom = res.register(self.atom());
        if res.error.is_some() {
            return res;
        }

        if self.current_token.typee == Tokens::LeftSquareBraces {
            res.register_advancement();
            self.advance();

            let index = res.register(self.expr());
            if res.error.is_some() {
                return res;
            }

            if self.current_token.typee != Tokens::RightSquareBraces {
                return res.failure(Error::new(
                    "Invalid Syntax",
                    pos_start,
                    self.current_token.pos_end.clone(),
                    "Expected ']'",
                ));
            }

            res.register_advancement();
            self.advance();

            return res.success(Node::ArrayAcess {
                array: Box::new(atom.unwrap()),
                index: Box::new(index.unwrap()),
            });
        } else if self.current_token.typee == Tokens::LeftParenthesis {
            let mut arg_nodes = vec![];
            res.register_advancement();
            self.advance();
            if self.current_token.typee == Tokens::RightParenthesis {
                res.register_advancement();
                self.advance();
            } else {
                let expr = res.register(self.expr());
                if res.error.is_some() {
                    return res.failure(Error::new(
                        "Invalid Syntax",
                        self.current_token.pos_start.clone(),
                        self.current_token.pos_end.clone(),
                        "Expected ')', 'var', int, float, identifier, '+', '-' or ','",
                    ));
                }
                arg_nodes.push(expr.unwrap());

                while self.current_token.typee == Tokens::Comma {
                    res.register_advancement();
                    self.advance();

                    let expr = res.register(self.expr());
                    if res.error.is_some() {
                        return res.failure(Error::new(
                            "Invalid Syntax",
                            self.current_token.pos_start.clone(),
                            self.current_token.pos_end.clone(),
                            "Expected ')', 'var', int, float, identifier, '+', '-' or ','",
                        ));
                    }
                    arg_nodes.push(expr.unwrap());
                }

                if self.current_token.typee != Tokens::RightParenthesis {
                    return res.failure(Error::new(
                        "Invalid Syntax",
                        self.current_token.pos_start.clone(),
                        self.current_token.pos_end.clone(),
                        "Expected ')' or ','",
                    ));
                }
                res.register_advancement();
                self.advance();
            }
            return res.success(Node::CallNode {
                node_to_call: Box::new(atom.clone().unwrap()),
                args: arg_nodes,
            });
        } else if self.current_token.typee == Tokens::Dot {
            self.advance();
            res.register_advancement();

            if self.current_token.typee != Tokens::Identifier {
                return res.failure(Error::new(
                    "Invalid Syntax",
                    self.current_token.pos_start.clone(),
                    self.current_token.pos_end.clone(),
                    "Expected identifier",
                ));
            }

            let mut id = self.current_token.clone();

            res.register_advancement();
            self.advance();

            if self.current_token.typee == Tokens::Equals {
                res.register_advancement();
                self.advance();

                let expr = res.register(self.expr());
                if res.error.is_some() {
                    return res;
                }

                return res.success(Node::ObjectPropEdit {
                    object: Box::new(atom.clone().unwrap()),
                    property: id,
                    new_val: Box::new(expr.unwrap()),
                });
            }

            let mut l = Node::ObjectPropAccess {
                object: Box::new(atom.clone().unwrap()),
                property: id,
            };

            while self.current_token.typee == Tokens::Dot {
                self.advance();
                res.register_advancement();

                if self.current_token.typee != Tokens::Identifier {
                    return res.failure(Error::new(
                        "Invalid Syntax",
                        self.current_token.pos_start.clone(),
                        self.current_token.pos_end.clone(),
                        "Expected identifier",
                    ));
                }

                id = self.current_token.clone();

                res.register_advancement();
                self.advance();

                if self.current_token.typee == Tokens::Equals {
                    res.register_advancement();
                    self.advance();

                    let expr = res.register(self.expr());
                    if res.error.is_some() {
                        return res;
                    }

                    return res.success(Node::ObjectPropEdit {
                        object: Box::new(l),
                        property: id,
                        new_val: Box::new(expr.unwrap()),
                    });
                }

                l = Node::ObjectPropAccess {
                    object: Box::new(l),
                    property: id,
                };
            }
            return res.success(l);
        }

        res.success(atom.unwrap())
    }
}
