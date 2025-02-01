<html>

<body style="font-family: monospace">
  <h3>Hey {{name}},</h3>
  <p>
  Forgot your password? No worries! You can reset it by clicking the link below:
  <a href="{{domain}}/api/auth/reset?token={{resetToken}}">Reset Your Password</a>
  If you didn't request a password reset, please ignore this email.
  </p>
</body>
</html>
