function validateForm(event) {
  event.preventDefault();

  const username = document.getElementById('username').value.trim();
  const email = document.getElementById('email').value.trim();
  const password = document.getElementById('password').value.trim();
  const confirmPassword = document.getElementById('confirm-password').value.trim();
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;

  if (!username || !email || !password || !confirmPassword) {
    alert('All fields are required!');
    return false;
  }

  if (!emailRegex.test(email)) {
    alert('Please enter a valid email address!');
    return false;
  }

  if (password !== confirmPassword) {
    alert('Passwords do not match!');
    return false;
  }

  document.getElementById("signupForm").submit();
}
 
document.addEventListener("DOMContentLoaded", () => {
  // Function to retrieve session_id from cookies
  function getSessionId() {
    const cookies = document.cookie.split("; ");
    for (const cookie of cookies) {
      const [name, value] = cookie.split("=");
      if (name === "session_id") {
        return value;
      }
    }
    return null;
  }

  // Check session_id and update the navigation link
  const sessionId = getSessionId();
  const signInLink = document.getElementById("signin-link");

  if (sessionId && signInLink != null) {
    // Change "Sign In" to "Sign Out"
    signInLink.href = "/signout";
    signInLink.textContent = "Sign Out";
  }
});

function validateEmail(email) {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return emailRegex.test(email);
}