// THIS IS AN AUTOGENERATED FILE. DO NOT EDIT THIS FILE DIRECTLY.

import {
  TypedMap,
  Entity,
  Value,
  ValueKind,
  store,
  Bytes,
  BigInt,
  BigDecimal
} from "@graphprotocol/graph-ts";

export class GraphNetwork extends Entity {
  constructor(id: string) {
    super();
    this.set("id", Value.fromString(id));
  }

  save(): void {
    let id = this.get("id");
    assert(id != null, "Cannot save GraphNetwork entity without an ID");
    if (id) {
      assert(
        id.kind == ValueKind.STRING,
        `Entities of type GraphNetwork must have an ID of type String but the id '${id.displayData()}' is of type ${id.displayKind()}`
      );
      store.set("GraphNetwork", id.toString(), this);
    }
  }

  static load(id: string): GraphNetwork | null {
    return changetype<GraphNetwork | null>(store.get("GraphNetwork", id));
  }

  get id(): string {
    let value = this.get("id");
    return value!.toString();
  }

  set id(value: string) {
    this.set("id", Value.fromString(value));
  }

  get totalTokensStaked(): BigInt | null {
    let value = this.get("totalTokensStaked");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toBigInt();
    }
  }

  set totalTokensStaked(value: BigInt | null) {
    if (!value) {
      this.unset("totalTokensStaked");
    } else {
      this.set("totalTokensStaked", Value.fromBigInt(<BigInt>value));
    }
  }

  get totalTokensDelegated(): BigInt | null {
    let value = this.get("totalTokensDelegated");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toBigInt();
    }
  }

  set totalTokensDelegated(value: BigInt | null) {
    if (!value) {
      this.unset("totalTokensDelegated");
    } else {
      this.set("totalTokensDelegated", Value.fromBigInt(<BigInt>value));
    }
  }

  get totalTokensSignalled(): BigInt | null {
    let value = this.get("totalTokensSignalled");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toBigInt();
    }
  }

  set totalTokensSignalled(value: BigInt | null) {
    if (!value) {
      this.unset("totalTokensSignalled");
    } else {
      this.set("totalTokensSignalled", Value.fromBigInt(<BigInt>value));
    }
  }

  get totalTokensSignalledAutoMigrate(): BigDecimal | null {
    let value = this.get("totalTokensSignalledAutoMigrate");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toBigDecimal();
    }
  }

  set totalTokensSignalledAutoMigrate(value: BigDecimal | null) {
    if (!value) {
      this.unset("totalTokensSignalledAutoMigrate");
    } else {
      this.set(
        "totalTokensSignalledAutoMigrate",
        Value.fromBigDecimal(<BigDecimal>value)
      );
    }
  }

  get totalTokensSignalledDirectly(): BigDecimal | null {
    let value = this.get("totalTokensSignalledDirectly");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toBigDecimal();
    }
  }

  set totalTokensSignalledDirectly(value: BigDecimal | null) {
    if (!value) {
      this.unset("totalTokensSignalledDirectly");
    } else {
      this.set(
        "totalTokensSignalledDirectly",
        Value.fromBigDecimal(<BigDecimal>value)
      );
    }
  }

  get totalSupply(): BigInt | null {
    let value = this.get("totalSupply");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toBigInt();
    }
  }

  set totalSupply(value: BigInt | null) {
    if (!value) {
      this.unset("totalSupply");
    } else {
      this.set("totalSupply", Value.fromBigInt(<BigInt>value));
    }
  }

  get totalGRTMinted(): BigInt | null {
    let value = this.get("totalGRTMinted");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toBigInt();
    }
  }

  set totalGRTMinted(value: BigInt | null) {
    if (!value) {
      this.unset("totalGRTMinted");
    } else {
      this.set("totalGRTMinted", Value.fromBigInt(<BigInt>value));
    }
  }

  get totalGRTBurned(): BigInt | null {
    let value = this.get("totalGRTBurned");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toBigInt();
    }
  }

  set totalGRTBurned(value: BigInt | null) {
    if (!value) {
      this.unset("totalGRTBurned");
    } else {
      this.set("totalGRTBurned", Value.fromBigInt(<BigInt>value));
    }
  }
}

export class GraphAccount extends Entity {
  constructor(id: string) {
    super();
    this.set("id", Value.fromString(id));
  }

  save(): void {
    let id = this.get("id");
    assert(id != null, "Cannot save GraphAccount entity without an ID");
    if (id) {
      assert(
        id.kind == ValueKind.STRING,
        `Entities of type GraphAccount must have an ID of type String but the id '${id.displayData()}' is of type ${id.displayKind()}`
      );
      store.set("GraphAccount", id.toString(), this);
    }
  }

  static load(id: string): GraphAccount | null {
    return changetype<GraphAccount | null>(store.get("GraphAccount", id));
  }

  get id(): string {
    let value = this.get("id");
    return value!.toString();
  }

  set id(value: string) {
    this.set("id", Value.fromString(value));
  }

  get balance(): BigInt | null {
    let value = this.get("balance");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toBigInt();
    }
  }

  set balance(value: BigInt | null) {
    if (!value) {
      this.unset("balance");
    } else {
      this.set("balance", Value.fromBigInt(<BigInt>value));
    }
  }

  get indexer(): string | null {
    let value = this.get("indexer");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toString();
    }
  }

  set indexer(value: string | null) {
    if (!value) {
      this.unset("indexer");
    } else {
      this.set("indexer", Value.fromString(<string>value));
    }
  }

  get delegator(): string | null {
    let value = this.get("delegator");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toString();
    }
  }

  set delegator(value: string | null) {
    if (!value) {
      this.unset("delegator");
    } else {
      this.set("delegator", Value.fromString(<string>value));
    }
  }

  get curator(): string | null {
    let value = this.get("curator");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toString();
    }
  }

  set curator(value: string | null) {
    if (!value) {
      this.unset("curator");
    } else {
      this.set("curator", Value.fromString(<string>value));
    }
  }
}

export class Indexer extends Entity {
  constructor(id: string) {
    super();
    this.set("id", Value.fromString(id));
  }

  save(): void {
    let id = this.get("id");
    assert(id != null, "Cannot save Indexer entity without an ID");
    if (id) {
      assert(
        id.kind == ValueKind.STRING,
        `Entities of type Indexer must have an ID of type String but the id '${id.displayData()}' is of type ${id.displayKind()}`
      );
      store.set("Indexer", id.toString(), this);
    }
  }

  static load(id: string): Indexer | null {
    return changetype<Indexer | null>(store.get("Indexer", id));
  }

  get id(): string {
    let value = this.get("id");
    return value!.toString();
  }

  set id(value: string) {
    this.set("id", Value.fromString(value));
  }

  get account(): string | null {
    let value = this.get("account");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toString();
    }
  }

  set account(value: string | null) {
    if (!value) {
      this.unset("account");
    } else {
      this.set("account", Value.fromString(<string>value));
    }
  }

  get stakedTokens(): BigInt | null {
    let value = this.get("stakedTokens");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toBigInt();
    }
  }

  set stakedTokens(value: BigInt | null) {
    if (!value) {
      this.unset("stakedTokens");
    } else {
      this.set("stakedTokens", Value.fromBigInt(<BigInt>value));
    }
  }

  get delegators(): Array<string> {
    let value = this.get("delegators");
    return value!.toStringArray();
  }

  set delegators(value: Array<string>) {
    this.set("delegators", Value.fromStringArray(value));
  }

  get delegatedTokens(): BigInt | null {
    let value = this.get("delegatedTokens");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toBigInt();
    }
  }

  set delegatedTokens(value: BigInt | null) {
    if (!value) {
      this.unset("delegatedTokens");
    } else {
      this.set("delegatedTokens", Value.fromBigInt(<BigInt>value));
    }
  }
}

export class Delegator extends Entity {
  constructor(id: string) {
    super();
    this.set("id", Value.fromString(id));
  }

  save(): void {
    let id = this.get("id");
    assert(id != null, "Cannot save Delegator entity without an ID");
    if (id) {
      assert(
        id.kind == ValueKind.STRING,
        `Entities of type Delegator must have an ID of type String but the id '${id.displayData()}' is of type ${id.displayKind()}`
      );
      store.set("Delegator", id.toString(), this);
    }
  }

  static load(id: string): Delegator | null {
    return changetype<Delegator | null>(store.get("Delegator", id));
  }

  get id(): string {
    let value = this.get("id");
    return value!.toString();
  }

  set id(value: string) {
    this.set("id", Value.fromString(value));
  }

  get account(): string | null {
    let value = this.get("account");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toString();
    }
  }

  set account(value: string | null) {
    if (!value) {
      this.unset("account");
    } else {
      this.set("account", Value.fromString(<string>value));
    }
  }

  get stakes(): Array<string> {
    let value = this.get("stakes");
    return value!.toStringArray();
  }

  set stakes(value: Array<string>) {
    this.set("stakes", Value.fromStringArray(value));
  }

  get totalStakedTokens(): BigInt | null {
    let value = this.get("totalStakedTokens");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toBigInt();
    }
  }

  set totalStakedTokens(value: BigInt | null) {
    if (!value) {
      this.unset("totalStakedTokens");
    } else {
      this.set("totalStakedTokens", Value.fromBigInt(<BigInt>value));
    }
  }
}

export class DelegatedStake extends Entity {
  constructor(id: string) {
    super();
    this.set("id", Value.fromString(id));
  }

  save(): void {
    let id = this.get("id");
    assert(id != null, "Cannot save DelegatedStake entity without an ID");
    if (id) {
      assert(
        id.kind == ValueKind.STRING,
        `Entities of type DelegatedStake must have an ID of type String but the id '${id.displayData()}' is of type ${id.displayKind()}`
      );
      store.set("DelegatedStake", id.toString(), this);
    }
  }

  static load(id: string): DelegatedStake | null {
    return changetype<DelegatedStake | null>(store.get("DelegatedStake", id));
  }

  get id(): string {
    let value = this.get("id");
    return value!.toString();
  }

  set id(value: string) {
    this.set("id", Value.fromString(value));
  }

  get indexer(): string | null {
    let value = this.get("indexer");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toString();
    }
  }

  set indexer(value: string | null) {
    if (!value) {
      this.unset("indexer");
    } else {
      this.set("indexer", Value.fromString(<string>value));
    }
  }

  get delegator(): string | null {
    let value = this.get("delegator");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toString();
    }
  }

  set delegator(value: string | null) {
    if (!value) {
      this.unset("delegator");
    } else {
      this.set("delegator", Value.fromString(<string>value));
    }
  }

  get stakedTokens(): BigInt | null {
    let value = this.get("stakedTokens");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toBigInt();
    }
  }

  set stakedTokens(value: BigInt | null) {
    if (!value) {
      this.unset("stakedTokens");
    } else {
      this.set("stakedTokens", Value.fromBigInt(<BigInt>value));
    }
  }
}

export class Curator extends Entity {
  constructor(id: string) {
    super();
    this.set("id", Value.fromString(id));
  }

  save(): void {
    let id = this.get("id");
    assert(id != null, "Cannot save Curator entity without an ID");
    if (id) {
      assert(
        id.kind == ValueKind.STRING,
        `Entities of type Curator must have an ID of type String but the id '${id.displayData()}' is of type ${id.displayKind()}`
      );
      store.set("Curator", id.toString(), this);
    }
  }

  static load(id: string): Curator | null {
    return changetype<Curator | null>(store.get("Curator", id));
  }

  get id(): string {
    let value = this.get("id");
    return value!.toString();
  }

  set id(value: string) {
    this.set("id", Value.fromString(value));
  }

  get account(): string | null {
    let value = this.get("account");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toString();
    }
  }

  set account(value: string | null) {
    if (!value) {
      this.unset("account");
    } else {
      this.set("account", Value.fromString(<string>value));
    }
  }

  get totalSignalledTokens(): BigInt | null {
    let value = this.get("totalSignalledTokens");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toBigInt();
    }
  }

  set totalSignalledTokens(value: BigInt | null) {
    if (!value) {
      this.unset("totalSignalledTokens");
    } else {
      this.set("totalSignalledTokens", Value.fromBigInt(<BigInt>value));
    }
  }

  get totalUnsignalledTokens(): BigInt | null {
    let value = this.get("totalUnsignalledTokens");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toBigInt();
    }
  }

  set totalUnsignalledTokens(value: BigInt | null) {
    if (!value) {
      this.unset("totalUnsignalledTokens");
    } else {
      this.set("totalUnsignalledTokens", Value.fromBigInt(<BigInt>value));
    }
  }

  get totalNameSignalledTokens(): BigInt | null {
    let value = this.get("totalNameSignalledTokens");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toBigInt();
    }
  }

  set totalNameSignalledTokens(value: BigInt | null) {
    if (!value) {
      this.unset("totalNameSignalledTokens");
    } else {
      this.set("totalNameSignalledTokens", Value.fromBigInt(<BigInt>value));
    }
  }

  get totalNameUnsignalledTokens(): BigInt | null {
    let value = this.get("totalNameUnsignalledTokens");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toBigInt();
    }
  }

  set totalNameUnsignalledTokens(value: BigInt | null) {
    if (!value) {
      this.unset("totalNameUnsignalledTokens");
    } else {
      this.set("totalNameUnsignalledTokens", Value.fromBigInt(<BigInt>value));
    }
  }
}