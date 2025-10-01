#!/bin/sh
# Copyright 2025 Philip Cronje
#
# This file is part of my Advent of Code solution repository. It is free software: you can
# redistribute it and/or modify it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or (at your option) any later
# version.
#
# This repository is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
# without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License along with this repository. If
# not, see <https://www.gnu.org/licenses/>.
set -o errexit
dotnet build --configuration=Release
JAVA_HOME=/usr/lib/jvm/java-25-openjdk ./gradlew build
exec ninja -C builddir
