// GraphQL query definitions
pub const SIGNUP_MUTATION: &str = r#"
query signUp($input: SignUpInput!) {
  signUp(input: $input)
}
"#;

pub const LOGIN_MUTATION: &str = r#"
query login($input: LoginInput!) {
  login(input: $input)
}
"#;

pub const CREATE_TEAM_MUTATION: &str = r#"
mutation createTeam($input: CreateTeamInput!) {
  createTeam(input: $input)
}
"#;

pub const CREATE_FORM_MUTATION: &str = r#"
mutation createForm($input: CreateFormInput!) {
  createForm(input: $input)
}
"#;

pub const UPDATE_FORM_MUTATION: &str = r#"
mutation updateForm($input: UpdateFormInput!) {
  updateForm(input: $input)
}
"#;

pub const UPDATE_FORM_THEME_MUTATION: &str = r#"
mutation updateFormTheme($input: UpdateFormThemeInput!) {
  updateFormTheme(input: $input)
}
"#;

pub const CREATE_FORM_HIDDEN_FIELD_MUTATION: &str = r#"
mutation createFormHiddenField($input: CreateHiddenFieldInput!) {
  createFormHiddenField(input: $input)
}
"#;

pub const DUPLICATE_FORM_MUTATION: &str = r#"
mutation duplicateForm($input: FormDetailInput!) {
  duplicateForm(input: $input)
}
"#;

pub const FORM_DETAIL_QUERY: &str = r#"
query formDetail($input: FormDetailInput!) {
  form(input: $input) {
    id
    teamId
    projectId
    name
    description
    interactiveMode
    kind
    settings {
      active
      published
      allowArchive
      locale
      enableQuestionList
    }
    fields {
      id
      title
      description
      kind
      validations
      properties
      layout
      width
      hide
      frozen
    }
    themeSettings {
      theme
    }
    draft
    status
  }
}
"#;

pub const USER_DETAIL_QUERY: &str = r#"
query {
  user {
    id
    name
    email
    avatar
    lang
    isEmailVerified
    isSocialAccount
  }
}
"#;

pub const TEAMS_QUERY: &str = r#"
query {
  teams {
    id
    name
    ownerId
    inviteCode
    avatar
    memberCount
    createdAt
    projects{
        id 
        teamId
        name
    }
  }
}
"#;

