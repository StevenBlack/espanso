/*
 * This file is part of espanso.
 *
 * Copyright (C) 2019-2021 Federico Terzi
 *
 * espanso is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * espanso is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with espanso.  If not, see <https://www.gnu.org/licenses/>.
 */

pub mod keyboard;
pub mod inject;
pub mod matches;
pub mod render;

#[derive(Debug)]
pub enum Event {
  NOOP,
  ProcessingError(String),
  
  // Inputs
  Keyboard(keyboard::KeyboardEvent),

  // Internal
  MatchesDetected(matches::MatchesDetectedEvent),
  MatchSelected(matches::MatchSelectedEvent),

  RenderingRequested(render::RenderingRequestedEvent),
  Rendered(render::RenderedEvent),

  // Effects
  TextInject(inject::TextInjectRequest),
}