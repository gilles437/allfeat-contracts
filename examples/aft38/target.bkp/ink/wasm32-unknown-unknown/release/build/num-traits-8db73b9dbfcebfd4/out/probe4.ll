; ModuleID = 'probe4.a5a8de2fde549693-cgu.0'
source_filename = "probe4.a5a8de2fde549693-cgu.0"
target datalayout = "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-n32:64-S128-ni:1:10:20"
target triple = "wasm32-unknown-unknown"

@alloc_15c1356ab95b0d2b98ea1d91ca33573b = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/eb26296b556cef10fb713a38f3d16b9886080f26/library/core/src/num/mod.rs" }>, align 1
@alloc_169079f12bfd1a7051253581e01c6c24 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_15c1356ab95b0d2b98ea1d91ca33573b, [12 x i8] c"K\00\00\00~\04\00\00\05\00\00\00" }>, align 4
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; probe4::probe
; Function Attrs: nounwind
define hidden void @_ZN6probe45probe17h60e7ddd0b92b4af0E() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h23a49153b8840ad5E.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h04b2835ef9b41d19E(ptr align 1 @str.0, i32 25, ptr align 4 @alloc_169079f12bfd1a7051253581e01c6c24) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h23a49153b8840ad5E.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare hidden i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn nounwind
declare dso_local void @_ZN4core9panicking5panic17h04b2835ef9b41d19E(ptr align 1, i32, ptr align 4) unnamed_addr #2

attributes #0 = { nounwind "target-cpu"="mvp" }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #2 = { cold noinline noreturn nounwind "target-cpu"="mvp" }
attributes #3 = { noreturn nounwind }
