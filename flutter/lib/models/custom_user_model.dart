import 'dart:async';
import 'dart:convert';

import 'package:bot_toast/bot_toast.dart';
import 'package:flutter/material.dart';
import 'package:flutter_hbb/common/hbbs/hbbs.dart';
import 'package:get/get.dart';
import 'package:http/http.dart' as http;

import '../common.dart';
import 'model.dart';
import 'platform_model.dart';

class CustomLoginResponse {
  String? userName;
  String? key;
  String? serverUrl;

  CustomLoginResponse({this.userName, this.key, this.serverUrl});

  CustomLoginResponse.fromJson(Map<String, dynamic> json) {
    userName = json['user_name'];
    key = json['key'];
    serverUrl = json['server_url'];
  }
}

class CustomUserModel {
  String? userName = "";
  String? password = "";

  CustomUserModel({this.userName, this.password});

  Future<CustomLoginResponse?> customLogin() async {
    try {
      final url = "http://127.0.0.1:3100";
      final response = await http.post(Uri.parse('$url/api/users/custom-login'),
          headers: {'Content-Type': 'application/json'},
          body: jsonEncode({
            'user_name': userName,
            'password': password,
          }));
      final status = response.statusCode;
      if (status == 401 || status == 400) {
        return null;
      }
      final data = json.decode(utf8.decode(response.bodyBytes));
      return CustomLoginResponse.fromJson(data);
    } catch (e) {
      debugPrint('Failed to Custom Login: $e');
    } finally {}
    return null;
  }
}
