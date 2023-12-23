#[doc = "Register `cpu_pll_cfg3` reader"]
pub struct R(crate::R<CPU_PLL_CFG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_PLL_CFG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_PLL_CFG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_PLL_CFG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cpu_pll_cfg3` writer"]
pub struct W(crate::W<CPU_PLL_CFG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_PLL_CFG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CPU_PLL_CFG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_PLL_CFG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cpupll_c4_en` reader - "]
pub type CPUPLL_C4_EN_R = crate::BitReader<bool>;
#[doc = "Field `cpupll_c4_en` writer - "]
pub type CPUPLL_C4_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPU_PLL_CFG3_SPEC, bool, O>;
#[doc = "Field `reserved_1_3` reader - "]
pub type RESERVED_1_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cpupll_r4` reader - "]
pub type CPUPLL_R4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cpupll_r4` writer - "]
pub type CPUPLL_R4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU_PLL_CFG3_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_6_7` reader - "]
pub type RESERVED_6_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cpupll_r4_short` reader - "]
pub type CPUPLL_R4_SHORT_R = crate::BitReader<bool>;
#[doc = "Field `cpupll_r4_short` writer - "]
pub type CPUPLL_R4_SHORT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPU_PLL_CFG3_SPEC, bool, O>;
#[doc = "Field `reserved_9_11` reader - "]
pub type RESERVED_9_11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cpupll_c3` reader - "]
pub type CPUPLL_C3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cpupll_c3` writer - "]
pub type CPUPLL_C3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU_PLL_CFG3_SPEC, u8, u8, 2, O>;
#[doc = "Field `cpupll_cz` reader - "]
pub type CPUPLL_CZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cpupll_cz` writer - "]
pub type CPUPLL_CZ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU_PLL_CFG3_SPEC, u8, u8, 2, O>;
#[doc = "Field `cpupll_rz` reader - "]
pub type CPUPLL_RZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cpupll_rz` writer - "]
pub type CPUPLL_RZ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU_PLL_CFG3_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_19_31` reader - "]
pub type RESERVED_19_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cpupll_c4_en(&self) -> CPUPLL_C4_EN_R {
        CPUPLL_C4_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn reserved_1_3(&self) -> RESERVED_1_3_R {
        RESERVED_1_3_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn cpupll_r4(&self) -> CPUPLL_R4_R {
        CPUPLL_R4_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn reserved_6_7(&self) -> RESERVED_6_7_R {
        RESERVED_6_7_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cpupll_r4_short(&self) -> CPUPLL_R4_SHORT_R {
        CPUPLL_R4_SHORT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn reserved_9_11(&self) -> RESERVED_9_11_R {
        RESERVED_9_11_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn cpupll_c3(&self) -> CPUPLL_C3_R {
        CPUPLL_C3_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn cpupll_cz(&self) -> CPUPLL_CZ_R {
        CPUPLL_CZ_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn cpupll_rz(&self) -> CPUPLL_RZ_R {
        CPUPLL_RZ_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:31"]
    #[inline(always)]
    pub fn reserved_19_31(&self) -> RESERVED_19_31_R {
        RESERVED_19_31_R::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cpupll_c4_en(&mut self) -> CPUPLL_C4_EN_W<0> {
        CPUPLL_C4_EN_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn cpupll_r4(&mut self) -> CPUPLL_R4_W<4> {
        CPUPLL_R4_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn cpupll_r4_short(&mut self) -> CPUPLL_R4_SHORT_W<8> {
        CPUPLL_R4_SHORT_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn cpupll_c3(&mut self) -> CPUPLL_C3_W<12> {
        CPUPLL_C3_W::new(self)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn cpupll_cz(&mut self) -> CPUPLL_CZ_W<14> {
        CPUPLL_CZ_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn cpupll_rz(&mut self) -> CPUPLL_RZ_W<16> {
        CPUPLL_RZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cpu_pll_cfg3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_pll_cfg3](index.html) module"]
pub struct CPU_PLL_CFG3_SPEC;
impl crate::RegisterSpec for CPU_PLL_CFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_pll_cfg3::R](R) reader structure"]
impl crate::Readable for CPU_PLL_CFG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_pll_cfg3::W](W) writer structure"]
impl crate::Writable for CPU_PLL_CFG3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cpu_pll_cfg3 to value 0x0005_a021"]
impl crate::Resettable for CPU_PLL_CFG3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0005_a021;
}
