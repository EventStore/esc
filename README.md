# EvenStoreDB Cloud CLI

`esc` in a command-line tool that allows to access EventStoreDB Cloud API in the confort of your terminal.

## Authentication

If you use `esc` for the first time, you need to generate a token first. By default, `esc` will interactively
ask your email and password to create your token.

If you prefer to have a non-interactive way to create your token, enter that command instead:

```
esc access tokens create --email <email> --unsafe-password <password>
```

`esc` will refresh your token automatically without you needing to do anything. Rest assured that
`esc` doesn't store your password in your system.

## Implicit parameters

Virtually all commands require `--org-id` and `--project-id` parameters. It is possible to tell
`esc` to use a preset `--org-id` or `--project-id`. You only need to create a local profile.

```
esc profiles set --profile <profile> --name <name> --value <value>
```

For example if you want to set a default `--org-id` for a profile named `my_profile`:

```
esc profiles set --profile my_profile --name org-id --value <my-org-id>
```

Similarly, if you want to set a default `--project-id` do:

```
esc profiles set --profile my_profile --name project-id --value <my-project-id>
```

Don't forget to set your local `my_profile` profile to be the default profile by doing the following:

```
esc profiles default set --value my_profile
```

From now, all the commands that need `--org-id` or `--project-id` will pick the value set in your
`my_profile` profile.

You can find more information about `profiles` by entering:

```
esc profiles --help
```

## Common usage examples:

### Create a network.

```
esc infra networks create  --cidr-block <cidr-block> --description <description> --provider <provider> --region <region>
```

If you want a description of each options, please do
```
esc infra networks create --help
```

### Create a peering link.

```
esc infra peerings create --org-id <org-id> --project-id <project-id> --description <description> --peer-account-id <peer-account-id> --peer-network-id <peer-network-id> --peer-network-region <peer-network-region>

```

If you want a description of each options, please do

```
esc infra peerings create --help
```

### Create a cluster.

```
esc mesdb clusters create --org-id <org-id> --project-id <project-id> --description <description> --disk-size-in-gb <disk-size-in-gb> --disk-type <disk-type> --instance-type <instance-type> --network-id <network-id> --projection-level <projection-level> --server-version <server-version> --topology <topology>
```

If you want a description of each options, please do:

```
esc mesdb clusters create --help
```