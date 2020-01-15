///////////////////////////////////////////////////////////////////////////////
//
//  Copyright 2018-2019 Airalab <research@aira.life> 
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//
///////////////////////////////////////////////////////////////////////////////
///! Rust generated ROS messages.
use rosrust::rosmsg_include;

rosmsg_include!(
    // Messages
    robonomics_msgs / Demand,
    robonomics_msgs / Offer,
    robonomics_msgs / Liability,
    robonomics_msgs / Report,
    // Services
    robonomics_msgs / StartLiability,
    robonomics_msgs / SendOrder,
    robonomics_msgs / SendReport,
);