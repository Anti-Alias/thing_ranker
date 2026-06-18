const redirectUri = 'http://localhost:3000/login';
const clientId = '675307479964-gkag44431kohr7pol8v11366q9aogahf.apps.googleusercontent.com';
const authBaseUrl = 'https://accounts.google.com/o/oauth2/v2/auth';
const authUrl = `${authBaseUrl}?client_id=${clientId}&response_type=code&scope=openid email&redirect_uri=${redirectUri}`;


function SignIn() {

  const signin = () => {
    window.location.href = authUrl;
  };

  return (
    <div>
      <h1>Sign In!!!</h1>
      <a href={authUrl}>
        <button> Sign in with Google</button>
      </a>
    </div >
  );
}

export default SignIn;
