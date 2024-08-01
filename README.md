# status-checker

Automated program for checking the health of networked services.

## Output Structure

The following TypeScript declarations document the structure of the program's output, with `Section` being the type of the object generated. Anything that consumes this API should be able to properly display any object that conforms to these types.

```ts
type State = 'Healthy' | 'Unhealthy' | 'Offline';

interface Status {
    state: State;
    text: string;
}

interface Service {
    name: string;
    desc: string;
    status: Status;
}

interface SubSection {
    name: string;
    desc: string;
    items: Item[];
}

type Item =
    | {
          Service: Service;
          SubSection?: never;
      }
    | {
          Service?: never;
          SubSection: SubSection;
      };

interface Section {
    time: number;
    overall_state: State;
    items: Item[];
}
```

## Running

The program is meant to be executed by a GitHub actions runner, but can be built and ran normally with cargo.

```sh
cargo run
```

Status is sent to `stdout` in JSON format.
