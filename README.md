# Reexport - Developed by Erik Medeiros

## Reason

A common pattern in the javascript world is a index file that reexport all other files in the same folder, an exemple can be the following file hierarchy

```
src/hooks
├── use-auth.ts
├── use-filter.ts
├── use-something-else.ts
└── index.ts
```

the index.ts content will be

```typescript
export * from './use-auth';
export * from './use-filter';
export * from './use-something-else';
```

when you have multiple folder with multiple files, manually reexporting each is very painfull, this package comes to solve this!

## To do

- better error handling
- better feedback to the user
- better readme?

## Usage

consider the following folder

```
src/components
├── component1.js
└── component2.ts
```

simply pass the path to the folder

```bash
reexport src/components
```

and a index.ts file will be created reexporting all files on that folder!

```typescript
// src/components/index.ts

export * from './component1';
export * from './component2';
```

you also can pass multiple paths to be reexported like

```bash
reexport src/components src/hooks
```

### Recursive flag

now let's take a look in a more complex file tree

```
src/components
├── a
│   ├── a1
│   │   ├── file2.ts
│   │   └── file.ts
│   ├── a2
│   │   └── aa1
│   │       ├── file2.js
│   │       └── file.ts
│   └── file.ts
├── b
│   ├── b1
│   │   └── file.ts
│   └── file.ts
├── c
│   └── file.ts
└── test.json
```

try running the command bellow

```bash
reexport src/components
```

it will create the file bellow

```typescript
// src/components/index.ts

// test.json was ignored!
export * from './a';
export * from './b';
export * from './c';
```

but notice that none of the exported folder has an index.ts file, making the created file useless. But do not worry! This can be solved by passing recursive flag

passing this flag will tell reexport to create an index.ts file for each subfolder that is reexported.

```bash
reexport src/components --recursive
```

executing the above command will result in a file hierarchy like the bellow

```
src/components
├── a
│   ├── a1
│   │   ├── file2.ts
│   │   ├── file.ts
│   │   └── index.ts
│   ├── a2
│   │   ├── aa1
│   │   │   ├── file2.js
│   │   │   ├── file.ts
│   │   │   └── index.ts
│   │   └── index.ts
│   ├── file.ts
│   └── index.ts
├── b
│   ├── b1
│   │   ├── file.ts
│   │   └── index.ts
│   ├── file.ts
│   └── index.ts
├── c
│   ├── file.ts
│   └── index.ts
├── index.ts
└── test.json
```

## Contributing

If you find this explanation confunsing or have some feature to be implemented, contact me or do a pull request (i don't know how this works)
