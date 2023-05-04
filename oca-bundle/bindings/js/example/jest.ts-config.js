module.exports = {
  preset: 'ts-jest',
  testEnvironment: 'node',
  roots: [
    "<rootDir>/test",
  ],
  moduleDirectories: [
    "node_modules",
  ],
  moduleNameMapper: {
    '^@test(.*)$': "<rootDir>test/$1",
  },
  clearMocks: true,
};
