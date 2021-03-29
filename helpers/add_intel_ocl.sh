#!/usr/bin/env bash
######################################################################
#
# add_intel_ocl.sh - Intel OpenCL installation helpers.
#
# Copyright 2020-2021 Naman Bishnoi
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
######################################################################
set -euxo
echo "**********************************************************************"
echo "Installing Intel OpenCL OneAPI Runtime Engine (2021.1)"
echo "**********************************************************************"

# Add GPG Keys
cd /tmp
curl -O https://apt.repos.intel.com/intel-gpg-keys/GPG-PUB-KEY-INTEL-SW-PRODUCTS.PUB
apt-key add GPG-PUB-KEY-INTEL-SW-PRODUCTS.PUB
rm GPG-PUB-KEY-INTEL-SW-PRODUCTS.PUB

# Add repository
echo "deb https://apt.repos.intel.com/oneapi all main" | tee /etc/apt/sources.list.d/oneAPI.list

# Install Intel OpenCL Runtime Engine
apt-get update
apt-get install -y intel-oneapi-runtime-opencl

echo "**********************************************************************"
echo "Installation Finished (Intel OpenCL Runtime)"
echo "**********************************************************************"
