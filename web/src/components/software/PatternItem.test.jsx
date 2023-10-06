/*
 * Copyright (c) [2023] SUSE LLC
 *
 * All Rights Reserved.
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of version 2 of the GNU General Public License as published
 * by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for
 * more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, contact SUSE LLC.
 *
 * To contact SUSE LLC about this file by physical or electronic mail, you may
 * find current contact information at www.suse.com.
 */

import React from "react";
import { screen } from "@testing-library/react";
import { installerRender } from "~/test-utils";

import cockpit from "../../lib/cockpit";

import { createClient } from "~/client";

import PatternItem from "./PatternItem";

jest.mock("~/client");
const addPatternFn = jest.fn().mockResolvedValue();
const removePatternFn = jest.fn().mockResolvedValue();
beforeEach(() => {
  createClient.mockImplementation(() => {
    return {
      software: {
        addPattern: addPatternFn,
        removePattern: removePatternFn,
      },
    };
  });
});

jest.mock("../../lib/cockpit");
const readFn = jest.fn().mockResolvedValue("");
const fileFn = jest.fn();
fileFn.mockImplementation(() => {
  return {
    read: readFn
  };
});
cockpit.file.mockImplementation(fileFn);

const pattern = {
  name: "yast2_basis",
  description: "YaST tools for basic system administration.",
  category: "Base Technologies",
  icon: "./yast",
  order: "1220",
  summary: "YaST Base Utilities"
};

describe("PatternItem", () => {
  it("displays the pattern summary and description", async () => {
    installerRender(<PatternItem pattern={pattern} />);

    // the summary is displayed
    screen.getByText(pattern.summary);
    // the description is displayed
    screen.getByText(pattern.description);
  });
});
